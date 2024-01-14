use bytes::BytesMut;
use serde_derive::{Deserialize, Serialize};
use serde_xdr::{from_bytes, to_bytes};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
    net::TcpStream,
};
use tracing::{event, Level};
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Deserialize, Serialize)]
pub enum Command{
    Ping,
    ListElements(Option<String>),
    Set(String, Option<String>),
    Get(String),
}


#[derive(Debug, Deserialize, Serialize)]
pub struct RequestPacket {
    pub cmd: Command,
}
impl RequestPacket {
    pub fn new(cmd: Command) -> RequestPacket {
        RequestPacket { cmd }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Value {
    Void,
    String(String),
    List(Vec<Value>),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponsePacket {
    pub val: Value,
}


pub fn trace_init() {
    let stdout_log = tracing_subscriber::fmt::layer()
        // .compact()
        //.without_time(); 
            ;
    // Start logging
    let subscriber = tracing_subscriber::Registry::default().with(stdout_log);
    tracing::subscriber::set_global_default(subscriber).expect("not ok");
    event!(Level::INFO, "Start");
}

#[derive(Debug)]
pub struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}
impl Connection {
    pub fn new(socket: TcpStream) -> Connection {
        Connection {
            stream: BufWriter::new(socket),
            buffer: BytesMut::with_capacity(4 * 1024),
        }
    }

    pub async fn write_request(&mut self, frame: &RequestPacket) -> crate::Result<()> {
        let src = to_bytes(frame)?;
        self.stream.write_all(&src).await?;
        self.stream.flush().await?;
        Ok(())
    }

    pub async fn read_request(&mut self) -> crate::Result<RequestPacket> {
        loop {
            if !self.buffer.is_empty() {
                return Ok(from_bytes(&mut self.buffer)?);
            }

            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                if self.buffer.is_empty() {
                    return Ok(RequestPacket::new(Command::Ping));
                } else {
                    return Err("connection reset by peer".into());
                }
            }
        }
    }

    pub async fn write_response(&mut self, frame: &ResponsePacket) -> crate::Result<()> {
        let src = to_bytes(frame)?;
        self.stream.write_all(&src).await?;
        self.stream.flush().await?;
        Ok(())
    }

    pub async fn read_response(&mut self) -> crate::Result<ResponsePacket> {
        loop {
            if !self.buffer.is_empty() {
                return Ok(from_bytes(&mut self.buffer)?);
            }

            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                if self.buffer.is_empty() {
                    let val = Value::Void;
                    return Ok(ResponsePacket{val});
                } else {
                    return Err("connection reset by peer".into());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    

    #[test]
    fn it_works() {}
}
