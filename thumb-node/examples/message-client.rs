use std::{fs::File, io::Write};

use pjl_error::AppError;
use processing_node::{ObjectID, Value};
use processing_node_client::dispatch::Dispatcher;

struct App {
    dispatcher: Dispatcher,
    dirs_ref: Option<Value>,
}

impl App {
    fn init(&mut self) {
        self.dirs_ref = Some(Value::Object(ObjectID {
            realm: "thumbs".into(),
            id: "Dirs".into(),
        }));
    }

    fn dirs_at(&self, id: i64) -> Value {
        if let Some(dirs_ref) = self.dirs_ref.as_ref() {
            let mut msg = dirs_ref.message("at:");
            msg.push_int(id);
            let obj = self.dispatcher.send(msg).unwrap();
            obj
        } else {
            Value::Void
        }
    }

    fn exec_unary(&self, receiver: Value, selector: &str) -> Value {
        let msg = receiver.message(selector);
        let obj = self.dispatcher.send(msg).unwrap();
        obj
    }
}

fn main() -> Result<(), AppError> {
    let mut app = App {
        dispatcher: Dispatcher::new()?,
        dirs_ref: None,
    };

    app.init();
    let dir = app.dirs_at(2);

    let files = app.exec_unary(dir, "files");

    println!("files: {files}");
    // println!("-> {:?}", obj);
    // let msg = obj.message("exists");
    // let v = app.dispatcher.send(msg)?;
    // println!("2-> {v:?}");

    // let mut msg = obj.message("thumbAt:");
    // msg.push_int(500);
    // if let Value::MimeData(mime, buf) = app.dispatcher.send(msg)? {
    //     assert_eq!("image/jpeg", mime);
    //     if let Ok(mut f) = File::create("demo.jpg") {
    //         f.write_all(&buf).unwrap();
    //     }
    // }

    Ok(())
}
