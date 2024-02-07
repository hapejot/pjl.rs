use std::{collections::BTreeMap, rc::Rc};

use dbx::{data::model::DataModel, error::Error, ser::{serialize_row, CopyRule}, SqlValue};

use serde_derive::Serialize;
use log::info;

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
    EMail { personid: String, address: String },
    Phone { personid: String, number: String },
}

#[test]
fn row_serializer() -> Result<(), Error> {
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
    let model = DataModel::new("person");

    let rs = serialize_row(Rc::new(model), p)?;
    println!("row serialized: {:#?}", rs);
    assert_eq!(4, rs.len());
    let r = &rs[0];
    assert_eq!(String::from(r.get("name").unwrap()), "Peter Jaeckel");
    assert_eq!(String::from(r.get("gender").unwrap()), "m");
    assert_eq!(r.get("age").unwrap(), &SqlValue::from(53));
    assert_eq!(r.table(), "person");
    Ok(())
}

#[derive(Serialize)]
struct Order {
    number: String,
    sold_to: Person,
}

#[test]
fn order_serializer() {
    let o = Order {
        number: "#100".to_string(),
        sold_to: Person {
            personid: "#2".to_string(),
            name: "Lizzy".to_string(),
            gender: Gender::Female,
            age: 21,
            identification: BTreeMap::from([("X".to_string(), "Y".to_string())]),
            communications: vec![Communication::EMail {
                personid: String::new(),
                address: "ab@c.de".into(),
            }],
        },
    };
    let model = DataModel::new("order");
    let _rule = CopyRule::new(vec![]);

    // let mut meta = Meta::new();

    // let rel = meta.define_relation(One, "Order", "sold_to", "Person");
    // meta.map_field(rel.as_str(), "sold_to_id", "personid");

    // let rel = meta.define_relation(Many, "Person", "communications", "EMail");
    // meta.map_field(rel.as_str(), "personid", "personid");

    // model.set_meta(meta);

    let rs = serialize_row(Rc::new(model), o).unwrap();
    assert_eq!(4, rs.len());
    for r in rs.iter() {
        match r.table() {
            "Order" => {
                info!("result row: {}", r);
                assert!(r.get("sold_to_id") == Some(&SqlValue::from("#2")));
            }
            "person" => info!("result row: {}", r),
            "EMail" => info!("result row: {}", r),
            "map" => info!("result row: {}", r),
            _ => panic!("unknown row type {}", r.table()),
        }
    }
}
