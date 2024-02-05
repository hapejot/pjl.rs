use std::cell::RefCell;
use std::fs::File;
use std::rc::Rc;

use dbx::data::model::FieldType::Lookup;
use dbx::data::model::FieldType::Text;
use dbx::data::Query;
use dbx::data::WhereCondition;
use dbx::data::WhereExpr;
use dbx::{
    data::model::{DataModel, Table},
    Database,
};
use serde::{Deserialize, Serialize};
use log::*;
// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Serialize, Deserialize)]
enum Gender {
    #[serde(rename = "m")]
    Male,
    Female,
    Other,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "person")]
struct Person {
    id: String,
    name1: String,
    name2: String,
    name3: String,
    name4: String,
    communications: Option<Vec<Communication>>,
    // identification: Option<BTreeMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
enum Communication {
    #[serde(rename = "email")]
    EMail { id: String, address: String },
    #[serde(rename = "phone")]
    Phone { id: String, number: String },
}

fn make_person_model() -> DataModel {
    let mut model = DataModel::new("Person");
    let tab = Table::new("person")
        .field("id", true, Text(20))
        .field("name1", false, Text(100))
        .field("name2", false, Text(100))
        .field("name3", false, Text(100))
        .field("name4", false, Text(100));
    model = model
        .table(tab)
        .table(
            Table::new("email")
                .field("id", true, Text(20))
                .field(
                    "person",
                    false,
                    Lookup {
                        table: "person".into(),
                        as_field: "communications".into(),
                    },
                )
                .field("role", false, Text(100))
                .field("address", false, Text(100)),
        )
        .table(
            Table::new("phone")
                .field("id", true, Text(20))
                .field(
                    "person",
                    false,
                    Lookup {
                        table: "person".into(),
                        as_field: "communications".into(),
                    },
                )
                .field("role", false, Text(100))
                .field("number", false, Text(100)),
        );
    // let mut meta = Meta::new();
    // meta.define_relation(One, "person", "Communication.email", "email");
    // meta.define_relation(One, "person", "Communication.phone", "phone");
    // model.set_meta(meta);
    model.build();
    model
}

fn main() {
    // let env_filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "trace".into());
    // tracing_subscriber::registry()
    //     .with(tracing_subscriber::EnvFilter::new(env_filter.clone()))
    //     .with(tracing_subscriber::fmt::layer())
    //     .init();
    env_logger::init();
    info!("start");
    

    let f = File::open("data.yaml").unwrap();
    let r = serde_yaml::from_reader::<_, Person>(f).unwrap();
    let model = make_person_model();
    let db = Database::new();
    db.connect("person.sqlite".into());
    db.activate_structure(model);

    db.modify_from_ser(&r).unwrap();

    read_record(db);
}

fn read_record(db: Database) {
    let condition = WhereCondition::new().and(WhereExpr::Equals("id".into(), "1".into()));
    let q = Query::new("person", vec!["*"], condition);

    let pp = db.select_rows(q);
    let buf = Rc::new(RefCell::new(String::new()));
    let fmt = edm::json::Format::new(buf.clone());
    fmt.convert(&pp.unwrap()).unwrap();
    println!("{}", buf.borrow());
}
