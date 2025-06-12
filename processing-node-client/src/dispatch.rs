use std::{
    cell::RefCell,
    collections::HashMap,
    fs::{read_to_string, File},
    net::TcpStream,
    path::PathBuf,
};
use tracing::*;
use pjl_error::AppError;
use processing_node::{config::Config, Message, ObjectID, Value};
use serde::{Deserialize, Serialize};

use crate::sync::PacketStream;

pub struct Dispatcher {
    server: HashMap<String, Connection>,
    config: Config,
}

impl Dispatcher {
    pub fn new() -> Result<Self, AppError> {
        let mut server = HashMap::new();
        #[cfg(debug_assertions)]
        let cfg_name = PathBuf::from(format!(
            "{}/.config/processing-node/config-test.yaml",
            std::env::var("HOME")?
        ));
        #[cfg(not(debug_assertions))]
        let cfg_name = PathBuf::from(format!(
            "{}/.config/processing-node/config.yaml",
            std::env::var("HOME")?
        ));
        let cfg_str = read_to_string(cfg_name)?;
        let config: Config = serde_yaml::from_str(cfg_str.as_str())?;
        for (k, v) in config.services.iter() {
            if let Ok(c) = Connection::new(&v) {
                server.insert(k.clone(), c);
            }
        }
        Ok(Self { server, config })
    }

    // pub fn lookup(&self, realm: &str, name: &str) -> Value {
    //     match self.server.get(realm) {
    //         Some(conn) => conn.lookup(name),
    //         None => panic!("unknown realm {realm}"),
    //     }
    // }

    pub fn send(&self, msg: Message) -> Result<Value, String> {
        let Message { receiver, .. } = &msg;
        let ObjectID { realm, .. } = receiver;
        match self.server.get(realm) {
            Some(con) => {
                let mut stream = con.stream.borrow_mut();
                match con.ps.write(&mut stream, &msg) {
                    Ok(_) => match con.ps.read::<Value>(&mut stream) {
                        Ok(v) => Ok(v),
                        Err(e) => Err(e),
                    },
                    Err(e) => Err(e),
                }
            }
            None => Err(format!("no service found for {realm}")),
        }

        // ps.write(&mut stream, &value).await.unwrap();

        // let v: Value = ps.read(&mut stream).await.unwrap();
    }
}

pub struct Connection {
    active: bool,
    ps: PacketStream,
    stream: RefCell<TcpStream>,
}

impl Connection {
    pub fn new(conn: &str) -> Result<Self, AppError> {
        let active = false;
        let stream = TcpStream::connect(conn).map_err(|e| format!("connect: {conn} -> {e}"))?;
        let ps = PacketStream::new();
        Ok(Self {
            active,
            stream: RefCell::new(stream),
            ps,
        })
    }

    fn lookup(&self, name: &str) -> Value {
        todo!()
    }
}

impl std::fmt::Debug for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Connection")
            .field("active", &self.active)
            .field("ps", &"&self.ps")
            .field("stream", &self.stream)
            .finish()
    }
}
