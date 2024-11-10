use bytes::BytesMut;
use obj::ObjectRef;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    signal,
};
use tracing::{error, info, trace};
use tracing_subscriber::{layer::SubscriberExt, registry::Data, util::SubscriberInitExt};
pub mod obj;
pub mod parser;
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

    pub fn realm(&self) -> &str {
        &self.realm
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    receiver: ObjectID,
    selector: String,
    params: Vec<Value>,
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
}

pub mod non_sync;
pub mod sync;

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
    Object(ObjectID),
}
impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Void => String::new(),
            Value::String(s) => s.clone(),
            Value::List(vec) => todo!(),
            Value::Object(object_id) => todo!(),
        }
    }
}

pub struct Node {
    realm: String,
    registry: HashMap<String, ObjectRef>,
    dispatch: Box<dyn MessageDispatch>,
    stream: non_sync::PacketStream,
}

impl Node {
    pub fn new(realm: &str, dispatch: Box<dyn MessageDispatch>) -> Self {
        let registry = HashMap::new();
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
            registry,
            dispatch,
            stream: non_sync::PacketStream::new(),
        }
    }

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
                                        error!("{x}");
                                        break;
                                    }
                                    _ => {}
                                }
                            }
                            Err(e) => {
                                error!("{}", e);
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
        let local_net = format!("0.0.0.0:3000");

        info!("run");
        match TcpListener::bind(local_net.clone()).await {
            Ok(listener) => {
                info!("listener is started on {}", local_net);
                let ctrl_c = signal::ctrl_c();
                tokio::select! {
                    _= self.handle_connection(listener) => {}
                    _ = ctrl_c => { info!("shutting down on ctrl-c.");}
                }
            }
            Err(e) => error!("bind: {local_net} <- {e}"),
        };
        Ok(())
    }
}
