mod application;
mod db;

use crate::application::APP;
use axum::extract::{Form, Path};
use axum::{
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use clap::Parser;
use db::SharedData;

use minijinja::{context, Environment};


use std::sync::Mutex;
use std::collections::HashMap;
use std::{
    fs,
    net::SocketAddr,
};
use tracing::{error, info, warn};

use lazy_static::lazy_static;
use pjl_proto::{trace_init, Command, Connection, RequestPacket, ResponsePacket, Value};
use tokio::{
    net::{TcpListener, TcpStream},
    signal,
};

struct Templates<'source> {
    env: Mutex<Environment<'source>>,
}

lazy_static! {
    static ref T: Templates<'static> = Templates::new();
}

impl<'source> Templates<'source> {
    pub fn new() -> Self {
        Self {
            env: Mutex::new(Environment::new()),
        }
    }

    pub fn render(&self, template_name: &str, ctx: minijinja::value::Value) -> Option<String> {
        if let Ok(x) = self.env.lock() {
            if let Ok(y) = x.get_template(template_name) {
                if let Ok(z) = y.render(ctx) {
                    Some(z)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn load(&self) {
        if let Ok(mut x) = self.env.lock() {
            x.add_template_owned("root", fs::read_to_string("root.html").expect("..."))
                .expect("...");
            x.add_template_owned(
                "redirect",
                fs::read_to_string("redirect.html").expect("..."),
            )
            .expect("...");
            x.add_template_owned("obj", fs::read_to_string("obj.html").expect("..."))
                .expect("...");
            x.add_template_owned("action", fs::read_to_string("action.html").expect("..."))
                .expect("...");
        }
    }
}

#[allow(dead_code)]
fn load_templates() {}

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Parser, Debug)] // requires `derive` feature
pub struct Args {
    // interface to bind the listener to
    #[clap(long, default_value = "0.0.0.0")]
    interface: String,

    // port to bind to
    #[clap(long, default_value = "5555")]
    port: String,

    #[clap(long)]
    // read zip file as data
    zip: Option<String>,
}

async fn run(listener: TcpListener, shared_db: SharedData) {
    loop {
        match listener.accept().await {
            Ok((socket, _)) => {
                info!("new connection {:?}", socket);
                let db = shared_db.clone();
                tokio::spawn(async move {
                    process(socket, db).await;
                });
            }
            Err(e) => error!("accept: {e}"),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    trace_init();

    let args = Args::parse();

    info!("args: {:?}", args);

    T.load();

    APP.init();
    APP.set(1, "Name".into(), application::Value::Str("Peter".into()));
    let new_objid = APP.new_obj();
    APP.set(1, "Properties".into(), application::Value::Obj(new_objid));
    APP.set(
        new_objid,
        "Interval".into(),
        application::Value::Str("10".into()),
    );
    APP.save_obj(1);
    APP.save_obj(new_objid);
    APP.dump();

    let shared_db = if let Some(fname) = args.zip {
        info!("using database {}", fname);
        SharedData::db_open(fname).await?
    } else {
        info!("using empty in memory database");
        SharedData::new()
    };

    let local_net = format!("{}:{}", args.interface, args.port);

    // build our application with a route
    let web_app = Router::new()
        .route("/dir/:objid", get(dir_get).post(dir_post))
        .route("/action", post(action))
        .route("/", get(root));

    // .route("/users", post(create_user));

    // let forever = task::spawn(async {
    //     let mut interval = time::interval(Duration::from_millis(10000));

    //     loop {
    //         interval.tick().await;
    //         info!("tick {:?}", APP.get(1, &"Initial".into()));
    //     }
    // });

    // run our app with hyper, listening globally on port 3000
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 3000));
    // Bind the listener to the address
    match TcpListener::bind(local_net.clone()).await {
        Ok(listener) => {
            let ctrl_c = signal::ctrl_c();
            tokio::select! {
            _ =  run(listener, shared_db) => { error!("processing ended.");}
            _ = ctrl_c => { info!("shutting down on ctrl-c.");}
            // _ = forever => { info!("timer..."); }
            _ = axum::Server::bind(&addr)
                        .serve(web_app.into_make_service()) => { info!("axum exited.");}
                   }
        }
        Err(e) => error!("bind: {local_net} <- {e}"),
    };
    Ok(())
}

async fn root() -> Html<String> {
    Html(T.render("root", context!(name => "Value")).unwrap())
}

async fn dir_get(Path(objid): Path<usize>) -> Html<String> {
    let mut values = Vec::new();
    let mut objids = Vec::new();
    let parts = APP.get_entries(objid);
    for (k, v) in parts {
        match v {
            application::Value::Str(s) => values.push((k, s)),
            application::Value::Obj(objid) => objids.push((k, objid)),
        }
    }
    let ctx = context!(  objid => objid, 
                                values => values, 
                                objs => objids);
    Html(T.render("obj", ctx).unwrap())
}

async fn dir_post(
    Path(objid): Path<usize>,
    Form(params): Form<HashMap<String, String>>,
) -> impl IntoResponse {
    let cmd = &params["@cmd"];
    let name = &params["@name"];
    for (key, y) in params.iter() {
        info!("{:} <- {:}", key, y);
        if key[0..1] != String::from("@") {
            APP.set(objid, key.clone(), application::Value::Str(y.clone()));
        }
    }
    if cmd == "new value" {
        APP.set(objid, name.clone(), application::Value::Str(String::from("")));
    }
    else if cmd == "new obj" {
        APP.set(objid, name.clone(), application::Value::Obj(APP.new_obj()));
    }
    APP.save_obj(objid);
    forward_to_obj_page(objid)
}

fn forward_to_obj_page(objid: usize) -> Html<String> {
    Html(
        T.render(
            "redirect",
            context!(target_location => format!("/dir/{:}", objid)),
        )
        .unwrap(),
    )
}

async fn action(Form(params): Form<HashMap<String, String>>) -> impl IntoResponse {
    info!("{:#?}", params);

    if let Some(cmd) = params.get("cmd") {
        match cmd.as_str() {
            "shutdown" => {
                warn!("shutting down service.");
            }
            _ => error!("unknown command {:}", cmd),
        }
    }

    Html(
        T.render("redirect", context!(target_location => "/"))
            .unwrap(),
    )
}

#[tracing::instrument(skip(socket, db))]
async fn process(socket: TcpStream, db: SharedData) {
    let mut connection = Connection::new(socket);

    let d = connection.read_request().await;

    match d {
        Ok(RequestPacket {
            cmd: Command::ListElements(pattern),
        }) => {
            info!("list elements {pattern:?}");
            let rv: Value = Value::Void;
            let res = ResponsePacket { val: rv };
            connection
                .write_response(&res)
                .await
                .expect("write response");
        }
        Ok(RequestPacket {
            cmd: Command::Set(key, value),
        }) => {
            {
                let mut data = db.r.d.lock().await;
                match value {
                    Some(s) => {
                        data.entries.insert(key, s);
                    }
                    None => {
                        data.entries.remove(&key);
                    }
                };
            };
            db.db_save().await.expect("...");
            let res = ResponsePacket { val: Value::Void };
            connection
                .write_response(&res)
                .await
                .expect("write response");
        }
        Ok(RequestPacket {
            cmd: Command::Get(key),
        }) => {
            let mut val = Value::Void;
            {
                let data = db.r.d.lock().await;
                if let Some(v) = data.entries.get(&key) {
                    val = Value::String(v.clone())
                }
            }
            let res = ResponsePacket { val };
            connection
                .write_response(&res)
                .await
                .expect("write response");
        }
        _ => info!("ping"),
    }
}
