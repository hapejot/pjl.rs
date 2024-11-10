use std::io::{Read, Write};

use bytes::BytesMut;
use processing_node::{Message, Node, non_sync::PacketStream, Value};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {



    let mut stream = TcpStream::connect("127.0.0.1:3000").await.unwrap();
    let ps = PacketStream::new();
    let mut value = Message::new("hello", "Directory", "new:");
    value.push_str("src");
    ps.write(&mut stream, &value).await.unwrap();

    let v: Value = ps.read(&mut stream).await.unwrap();
    println!("-> {:?}", v);
    match v {
        Value::Object(object_id) => {
            let mut msg = Message::new(object_id.realm(), object_id.id(), "files");
            ps.write(&mut stream, &msg).await.unwrap();
            println!("-> {:?}", ps.read::<Value>(&mut stream).await.unwrap());
        }
        _ => todo!(),
    }
}
