use std::{fmt::Display, future::Future};

use config::read_config;
use pjl_error::AppError;
// use obj::ObjectRef;
use serde::{de::value, Deserialize, Serialize};
// use std::collections::HashMap;
// use tokio::{
//     net::TcpListener,
//     signal,
// };
use tracing::{error, info, instrument, trace};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[cfg(feature = "script")]
pub mod obj;

#[cfg(feature = "server")]
pub mod non_sync;
#[cfg(feature = "script")]
pub mod parser;
#[cfg(feature = "script")]
pub mod sources;

pub trait MessageDispatch {
    fn dispatch(&mut self, reciever: usize, selector: &str, args: &[Value]) -> Value;
    fn resolve_id(&self, name: &str) -> usize;
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ObjectID {
    pub realm: String,
    pub id: String,
}

impl ObjectID {
    pub fn new(realm: &str, id: &str) -> Self {
        Self {
            realm: realm.into(),
            id: id.into(),
        }
    }

    pub fn new_value(realm: &str, id: &str) -> Value {
        let oid = Self::new(realm, id);
        Value::Object(oid)
    }

    pub fn realm(&self) -> &str {
        &self.realm
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub receiver: ObjectID,
    pub selector: String,
    pub params: Vec<Value>,
}

impl Message {
    pub fn new(realm: &str, id: &str, selector: &str) -> Self {
        let selector = selector.to_string();
        let realm = realm.to_string();
        let id = id.to_string();
        let params = vec![];
        Self {
            receiver: ObjectID { realm, id },
            selector,
            params,
        }
    }

    pub fn push_str(&mut self, arg: &str) {
        self.params.push(Value::String(arg.into()));
    }

    pub fn push_int(&mut self, arg: i64) {
        self.params.push(Value::Integer(arg));
    }

    pub fn push_bool(&mut self, arg: bool) {
        self.params.push(Value::Boolean(arg));
    }
}

#[allow(dead_code)]
fn dump(hd: &[u8]) {
    for x in hd {
        eprint!(" {x:02X}");
    }
    eprintln!();
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Value {
    Void,
    String(String),
    List(Vec<Value>),
    MimeData(String, Vec<u8>),
    Object(ObjectID),
    Boolean(bool),
    Integer(i64),
}
impl Value {
    pub fn message(&self, selector: &str) -> Message {
        match self {
            Value::Object(object_id) => Message {
                receiver: object_id.clone(),
                selector: selector.into(),
                params: vec![],
            },
            _ => todo!("Impl of message function for {:?} missing", self),
        }
    }

    pub fn to_int(&self) -> Result<i64, AppError> {
        match self {
            Value::String(s) => s.parse::<i64>().map_err(|x| x.into()),
            Value::Integer(v) => Ok(*v),
            _ => todo!(),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Void => write!(f, ""),
            Value::String(s) => write!(f, "{s}"),
            Value::Integer(n) => write!(f, "{n}"),
            Value::List(v) => {
                for x in v.iter() {
                    writeln!(f, "{x}")?;
                }
                Ok(())
            }
            Value::Object(ObjectID { realm, id }) => write!(f, "<{id}@{realm}>"),
            _ => todo!("display of {:?}", self),
        }
    }
}

#[cfg(feature = "server")]
pub struct Node {
    realm: String,
    // registry: HashMap<String, ObjectRef>,
    dispatch: Box<dyn MessageDispatch>,
    stream: non_sync::PacketStream,
}

#[cfg(feature = "server")]
use tokio::{net::TcpListener, signal};
#[cfg(feature = "server")]
impl Node {
    pub fn new(realm: &str, dispatch: Box<dyn MessageDispatch>) -> Self {
        // let registry = HashMap::new();
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "trace".into()),
            )
            .with(tracing_subscriber::fmt::layer())
            .init();
        info!("starting node {}", realm);

        Self {
            realm: realm.into(),
            // registry,
            dispatch,
            stream: non_sync::PacketStream::new(),
        }
    }

    #[instrument(skip_all)]
    async fn handle_connection(&mut self, listener: TcpListener) {
        loop {
            match listener.accept().await {
                Ok((mut socket, _)) => {
                    info!("new connection {:?}", socket);
                    loop {
                        let q = self.stream.read(&mut socket).await;
                        match q {
                            Ok(Message {
                                receiver,
                                selector,
                                params,
                            }) => {
                                trace!("{:?} {} {:#?}", receiver, selector, params);
                                let n = match receiver.id.parse::<usize>() {
                                    Ok(x) => x,
                                    Err(_) => self.dispatch.resolve_id(&receiver.id),
                                };
                                trace!("resolve id {} -> {}", receiver.id, n);
                                let value = self.dispatch.dispatch(n, &selector, &params);
                                match self.stream.write(&mut socket, &value).await {
                                    Err(x) => {
                                        error!("stream write: {x}");
                                        break;
                                    }
                                    _ => {}
                                }
                            }
                            Err(e) => {
                                error!("stream read: {}", e);
                                break;
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("accept: {e}");
                    break;
                }
            }
        }
    }

    pub async fn run(&mut self) -> Result<(), String> {
        let cfg = read_config();
        if let Some(svc) = cfg.services.get(&self.realm) {
            info!("run");
            match TcpListener::bind(&svc).await {
                Ok(listener) => {
                    info!("listener is started on {}", svc);
                    let ctrl_c = signal::ctrl_c();
                    tokio::select! {
                        _= self.handle_connection(listener) => {}
                        _ = ctrl_c => { info!("shutting down on ctrl-c.");}
                    }
                }
                Err(e) => error!("bind: {svc} <- {e}"),
            };
            Ok(())
        } else {
            Err(format!(
                "service {} was not definied in the config file",
                self.realm
            ))
        }
    }
}

pub mod config {
    use std::{collections::HashMap, fs::read_to_string, path::PathBuf};

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct Config {
        pub trace: Option<String>,
        pub database: String,
        pub services: HashMap<String, String>,
    }

    pub fn read_config() -> Config {
        #[cfg(debug_assertions)]
        let cfg_name = PathBuf::from(format!(
            "{}/.config/processing-node/config-test.yaml",
            std::env::var("HOME").unwrap()
        ));
        #[cfg(not(debug_assertions))]
        let cfg_name = PathBuf::from(format!(
            "{}/.config/processing-node/config.yaml",
            std::env::var("HOME").unwrap()
        ));
        let cfg_str = read_to_string(cfg_name).unwrap();
        let config: Config = serde_yaml::from_str(cfg_str.as_str()).unwrap();
        config
    }
}
