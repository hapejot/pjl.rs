use std::{
    fs::{read_dir, ReadDir},
    path::Path,
};

use processing_node::{MessageDispatch, Node, ObjectID, Value};
use tracing::debug;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

struct Hello {
    paths: Vec<String>,
}

impl Hello {
    fn new() -> Self {
        let paths = vec![];
        Self { paths }
    }
}

impl MessageDispatch for Hello {
    fn dispatch(&mut self, id: usize, selector: &str, args: &[Value]) -> Value {
        match selector {
            "new:" => match id {
                2 => {
                    let p = match &args[0] {
                        Value::String(s) => s.clone(),
                        _ => todo!(),
                    };

                    self.paths.push(p);
                    Value::Object(ObjectID::new(
                        "hello",
                        &format!("{}", self.paths.len() + 10),
                    ))
                }
                _ => todo!(),
            },
            "hello" => match id {
                1 => Value::String(format!("Hello {} {:?}", selector, args)),
                _ => todo!(),
            },
            "files" => {
                if id > 10 {
                    let path = Path::new(&self.paths[id - 11]);
                    let mut r = vec![];
                    for x in read_dir(path).unwrap() {
                        r.push(Value::String(
                            x.unwrap().file_name().to_str().unwrap().into(),
                        ));
                    }
                    Value::List(r)
                } else {
                    todo!()
                }
            }
            _ => todo!(),
        }
    }

    fn resolve_id(&self, name: &str) -> usize {
        match name {
            "Main" => 1,
            "Directory" => 2,
            _ => todo!(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let mut n = Node::new("hello", Box::new(Hello::new()));
    n.run().await?;
    Ok(())
}
