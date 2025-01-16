use pjl_error::AppError;
use processing_node::{ObjectID, Value};
use processing_node_client::dispatch::Dispatcher;

fn main() -> Result<(),AppError>{
    let dispatcher = Dispatcher::new()?;
    let obj_ref = Value::Object(ObjectID {
        realm: "hello".into(),
        id: "Directory".into(),
    });
    let mut msg = obj_ref.message("new:");
    msg.push_str("src");
    let v = dispatcher.send(msg).unwrap();
    println!("-> {:?}", v);
    let msg = v.message("files");
    let v = dispatcher.send(msg);
    println!("2-> {v:?}");
    Ok(())
}
