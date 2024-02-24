use std::collections::BTreeMap;

use edm::{structure::StructureValue, value::Value};
use log::*;
use serde_derive::Serialize;

use dbx::{
    data::model::FieldType::Lookup,
    data::model::FieldType::Text,
        data::model::DataModel,
    data::model::{
        meta::{Meta, RelationKind::One},
        Table,
    },
    data::*,
    Database, DatabaseBuilder,
};

#[test]
fn modify_from() {
    let db = prepare_database_object();

    let mut s = StructureValue::new_with_type("object");
    s["id"] = "1".into();
    s["type"]= "Null".into();
    // assert_eq!(s.keys(), vec!["id", "type"]);
    let s = Value::StructureValue(s);
    db.modify_from("object".into(), &s);
    assert!(db.is_connected());
}

fn prepare_database_object() -> Database {
    let model = DataModel::new("object").table(
        Table::new("object")
            .field("id", true, Text(20))
            .field("type", false, Text(20))
            .field("flags", false, Text(20)),
    );

    let builder = DatabaseBuilder::new();
    let db = builder.build();
    db.connect(None);
    db.activate_structure(model);
    db
}

#[test]
fn select() {
    let db = prepare_database_object();

    let q = Query::new(
        "object",
        vec!["id", "type"],
        WhereCondition::new().and(WhereExpr::Equals("type".into(), "Null".into())),
    );

    let res: Vec<BTreeMap<String, String>> = db.select(q);
    trace!("result: {:?}", res);
}

#[derive(Debug, Serialize)]
#[serde(rename = "communication")]
pub enum Communication {
    #[serde(rename = "phone")]
    Phone {
        id: Option<String>,
        number: String,
        role: String,
    },
    #[serde(rename = "email")]
    EMail {
        id: Option<String>,
        address: String,
        role: String,
    },
}

pub fn new_guid() -> String {
    uuid::Uuid::new_v4().to_string()
}

#[derive(Debug, Serialize)]
#[serde(rename = "person")]
struct Person {
    id: String,
    name1: String,
    name2: String,
    communications: Vec<Communication>,
    name3: Option<String>,
    name4: Option<String>,
}

#[test]
fn serialize() {
    simple_logging::log_to_stderr(log::LevelFilter::Trace);
    debug!("test serialize");
    let p = Person {
        name1: "Peter".to_string(),
        name2: "Jaeckel".to_string(),
        name3: Some("Freiherr".to_string()),
        name4: None,
        communications: vec![
            Communication::Phone {
                number: "+4912345".to_string(),
                role: "fake".to_string(),
                id: Some(new_guid()),
            },
            Communication::EMail {
                address: "a@bc.de".to_string(),
                role: "dummy".to_string(),
                id: Some(new_guid()),
            },
        ],
        id: new_guid(),
    };

    let db = prepare_person_db();
    assert!(db.is_connected());
    trace!("** modify *******************************************");
    db.modify_from_ser(&p).unwrap();
    trace!("** select *******************************************");
    let res = db.execute_query("select * from person");
    assert_eq!(1, res.len());
    for x in res {
        info!("person: {}", x);
    }

    let res = db.execute_query("select * from email");
    assert_eq!(1, res.len());
    for x in res.iter() {
        info!("email: {}", x);
    }

    let res = db.execute_query("select * from phone");
    assert_eq!(1, res.len());
    for x in res {
        info!("phone: {}", x);
    }
}

fn prepare_person_db() -> dbx::Database {
    let builder = DatabaseBuilder::new();
    let _copy_rule_1 = dbx::ser::CopyRule::new(vec![dbx::ser::FieldCopyRule {
        source: "id".to_string(),
        target: "personid".to_string(),
    }]);
    let model = make_person_model();
    let db = builder.build();
    db.connect(None);

    db.activate_structure(model);
    db
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
    let mut meta = Meta::new();
    meta.define_relation(One, "person", "communication.email", "email");
    meta.define_relation(One, "person", "communication.phone", "phone");
    // model.set_meta(meta);
    model.build();
    model
}

#[test]
fn test_new_model() {
    let model = make_person_model();
    let db = Database::new();
    db.connect(None);
    db.activate_structure(model.clone());
    for t in db.tables() {
        info!("table: {}", t);
    }
    // check if we can activate the same model again without errors
    db.activate_structure(model);
}
