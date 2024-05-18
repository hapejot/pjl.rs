use std::{collections::BTreeMap, rc::Rc};

use dbx::{
    data::model::{
        DataModel,
        FieldType::{Lookup, Text},
        Table,
    },
    error::Error,
    ser::serialize_row,    
};

use log::info;
use serde_derive::Serialize;
fn make_person_model() -> DataModel {
    let mut model = DataModel::new("Person");
    let tab = Table::new("person")
        .field("personid", true, Text(20))
        .field("name", false, Text(100))
        .field("gender", false, Text(100))
        .field("age", false, Text(100))
        .field(
            "identification",
            false,
            Lookup {
                table: "identification".into(),
                as_field: "A".into(),
            },
        );
    model = model
        .table(tab)
        .table(Table::new("email").field("address", true, Text(100)).field(
            "personid",
            false,
            Lookup {
                table: "person".into(),
                as_field: "communications".into(),
            },
        ))
        .table(Table::new("order").field("number", true, Text(100)).field(
            "sold_to",
            false,
            Lookup {
                table: "person".into(),
                as_field: "personid".into(),
            },
        ))
        .table(Table::new("phone").field("number", true, Text(100)).field(
            "personid",
            false,
            Lookup {
                table: "person".into(),
                as_field: "communications".into(),
            },
        ))
        .table(Table::new("identification").field("A", true, Text(100)));
    // let mut meta = Meta::new();
    // meta.define_relation(One, "person", "communication.email", "email");
    // meta.define_relation(One, "person", "communication.phone", "phone");
    // model.set_meta(meta);
    model.build();
    model.dump();
    model
}

#[allow(dead_code)]
#[derive(Serialize)]
enum Gender {
    #[serde(rename = "m")]
    Male,
    Female,
    Other,
}

#[derive(Serialize)]
#[serde(rename = "person")]
struct Person {
    personid: String,
    name: String,
    gender: Gender,
    age: u8,
    communications: Vec<Communication>,
    identification: BTreeMap<String, String>,
}

#[derive(Serialize)]
enum Communication {
    #[serde(rename = "email")]
    EMail { personid: String, address: String },
    #[serde(rename = "phone")]
    Phone { personid: String, number: String },
}

#[test]
fn row_serializer() -> Result<(), Error> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Trace)
        .init();
    let p = Person {
        personid: "#1".into(),
        name: "Peter Jaeckel".into(),
        gender: Gender::Male,
        age: 53,
        identification: BTreeMap::from([("A".to_string(), "B".to_string())]),
        communications: vec![
            Communication::EMail {
                personid: String::new(),
                address: "a@bc.de".into(),
            },
            Communication::Phone {
                personid: String::new(),
                number: "1234".into(),
            },
        ],
    };
    let model = make_person_model();
    let rs = serialize_row(Rc::new(model), p)?;
    assert_eq!(4, rs.len());
    let r = &rs[0];
    assert_eq!(r["name"].to_string(), "Peter Jaeckel");
    assert_eq!(r["gender"].to_string(), "m");
    // assert_eq!(r["age"], 53.into());
    // assert_eq!(r.table(), "person");
    Ok(())
}

#[derive(Serialize)]
#[serde(rename = "order")]
struct Order {
    number: String,
    sold_to: Person,
}

#[test]
fn order_serializer() {
    // env_logger::Builder::try
    // env_logger::Builder::from_default_env()
    //     .filter_level(log::LevelFilter::Trace)
    //     .init();
    let o = Order {
        number: "#100".to_string(),
        sold_to: Person {
            personid: "#2".to_string(),
            name: "Lizzy".to_string(),
            gender: Gender::Female,
            age: 21,
            identification: BTreeMap::from([("A".to_string(), "Y".to_string())]),
            communications: vec![Communication::EMail {
                personid: String::new(),
                address: "ab@c.de".into(),
            }],
        },
    };
    let model = make_person_model();

    let rs = serialize_row(Rc::new(model), o).unwrap();
    assert_eq!(4, rs.len());
    for r in rs.iter() {
        match r.datatype() {
            "Order" => {
                info!("result row: {}", r);
                // assert_eq!(r["sold_to_id"],Some(&SqlValue::from("#2")));
            }
            "person" => info!("result row: {}", r),
            "email" => info!("result row: {}", r),
            "map" => info!("result row: {}", r),
            "order" => info!("result row: {}", r),
            "identification" => info!("result row: {}", r),
            _ => panic!("unknown row type {}", r.datatype()),
        }
    }
}
