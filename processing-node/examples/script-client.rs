#[cfg(feature = "script")]
mod script_client {
    use std::{
        cell::{Cell, RefCell},
        collections::HashMap,
        fs::File,
        io::Read,
        net::TcpStream,
        path::Path,
        rc::Rc,
    };

    use clap::{Parser as ClapParser, ValueEnum};
    use processing_node::{
        parser::{
            rbparser::{self, Node as STNode, Parser},
            scanner::{chunks::ChunkedLexer, Scanner},
        },
        sources::{self, *},
        sync::PacketStream,
        Message, MessageDispatch, Node, ObjectID, Value,
    };
    use santiago::lexer::lex;

    /// Doc comment
    #[derive(ClapParser, Debug, Clone)]
    struct Cli {
        path: String,
        #[arg(long)]
        dump_tokens: bool,
        #[arg(long, default_value = "plain")]
        format: Format,
    }

    trait MessageHandler {
        fn send(&mut self, receiver_id: &str, selector: &str, params: Vec<Value>) -> Value;
    }

    struct LocalHandler {}

    impl LocalHandler {
        fn new() -> Self {
            Self {}
        }
    }

    struct RemoteHandler {
        ps: PacketStream,
        stream: TcpStream,
    }

    impl RemoteHandler {
        fn new() -> Self {
            let stream = TcpStream::connect("127.0.0.1:3000").unwrap();
            let ps = PacketStream::new();

            Self { ps, stream }
        }
    }

    impl MessageHandler for LocalHandler {
        fn send(&mut self, receiver_id: &str, selector: &str, params: Vec<Value>) -> Value {
            match selector {
                "at:get:" => match receiver_id {
                    "1" => {
                        let realm = params[0].to_string();
                        let id = params[1].to_string();
                        Value::Object(ObjectID { realm, id })
                    }
                    other => todo!("{other}"),
                },
                "display:" => match receiver_id {
                    "2" => {
                        println!("{:?}", params);
                        Value::Void
                    }
                    other => todo!("display: not implemented for object {other}."),
                },
                _ => todo!("local selector: {}", selector),
            }
        }
    }
    impl MessageHandler for RemoteHandler {
        fn send(&mut self, receiver_id: &str, selector: &str, params: Vec<Value>) -> Value {
            let mut value = Message::new("hello", receiver_id, &selector);
            for x in params.iter() {
                match x {
                    Value::String(s) => value.push_str(&s),
                    _ => todo!(),
                }
            }
            self.ps.write(&mut self.stream, &value).unwrap();

            self.ps.read(&mut self.stream).unwrap()
        }
    }

    struct App {
        vars: HashMap<String, Value>,
        handlers: HashMap<&'static str, Box<dyn MessageHandler>>,
    }

    impl App {
        fn new() -> Self {
            let mut vars = HashMap::new();
            vars.insert(
                "Remote".into(),
                Value::Object(ObjectID {
                    realm: "local".into(),
                    id: "1".into(),
                }),
            );
            vars.insert(
                "Console".into(),
                Value::Object(ObjectID {
                    realm: "local".into(),
                    id: "2".into(),
                }),
            );
            let mut handlers: HashMap<&'static str, Box<dyn MessageHandler>> = HashMap::new();
            handlers.insert("local", Box::new(LocalHandler::new()));
            handlers.insert("hello", Box::new(RemoteHandler::new()));
            Self { vars, handlers }
        }

        pub fn eval_stmts(&mut self, t: STNode) {
            match t {
                STNode::Statements(stmts) => {
                    for stmt in stmts {
                        match stmt {
                            STNode::Assign { target, value } => {
                                let v = self.eval(&value);
                                self.vars.insert(target, v);
                            }
                            _ => {
                                self.eval(&stmt);
                            }
                        }
                    }
                }
                STNode::None => {}
                _ => todo!(),
            }
        }

        fn eval(&mut self, n: &STNode) -> Value {
            match n {
                STNode::Expression { receiver, messages } => {
                    if messages.len() == 1 {
                        match &messages[0] {
                            STNode::Message {
                                selector,
                                params,
                                outer: cascade,
                            } => {
                                let o = self.eval(&receiver);
                                match o {
                                    Value::Object(ObjectID { realm, id }) => {
                                        let args = params.iter().map(|x| self.eval(x)).collect();
                                        let handler = self
                                            .handlers
                                            .get_mut(realm.as_str())
                                            .expect(&format!("no handler for {realm}"));
                                        let x = handler.send(id.as_str(), &selector, args);
                                        println!("{:?}", x);
                                    }
                                    x => todo!("{:?}", x),
                                }
                            }
                            _ => todo!(),
                        }
                    }
                    // let o = self.eval(&receiver);
                    // match o {
                    //     Value::Object(ObjectID { realm, id }) => {
                    //         let args = params.iter().map(|x| self.eval(x)).collect();
                    //         let handler = self
                    //             .handlers
                    //             .get_mut(realm.as_str())
                    //             .expect(&format!("no handler for {realm}"));
                    //         handler.send(id.as_str(), selector, args)
                    //     }
                    //     x => todo!("{:?}", x),
                    // }
                    Value::Void
                }
                STNode::Identifier(k) => match self.vars.get(k) {
                    Some(v) => v.clone(),
                    None => todo!("{}", k),
                },
                STNode::String(s) => Value::String(s.clone()),
                STNode::Symbol(s) => Value::String(s.clone()),
                x => todo!("{:?}", x),
            }
        }
    }

    fn main() {
        let args = Cli::parse();

        let mut dst = String::new();
        let mut f = File::open(&args.path).unwrap();
        f.read_to_string(&mut dst).unwrap();
        match parse(args.format, args.dump_tokens, Path::new(&args.path), &dst) {
            Ok(sources) => {
                let mut tui = processing_node::sources::browser::ui::Application::init(sources);
                match tui.run() {
                    Ok(_) => println!("ok"),
                    Err(e) => println!("error: {e}"),
                }
            }
            Err(e) => println!("ERROR: {}", e),
        }
    }
}

#[cfg(not(feature = "script"))]
fn main() {
    
}