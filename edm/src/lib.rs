pub mod atom;
pub mod json;
pub mod list;
pub mod number;
pub mod primitive;
pub mod structure;
pub mod value;

pub use csdl::Schema;

pub mod csdl {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Default)]
    pub struct Schema {
        pub annotations: Vec<Annotation>,
        pub associations: Vec<Association>,
        pub complex_types: Vec<ComplexType>,
        pub entity_containers: Vec<EntityContianer>,
        pub entity_types: Vec<EntityType>,
        pub enum_types: Vec<EnumType>,
        pub functions: Vec<Function>,
        pub usings: Vec<Using>,
        pub value_terms: Vec<ValueTerm>,
    }

    impl Schema {
        pub fn new() -> Self {
            let annotations = vec![];
            let associations = vec![];
            let complex_types = vec![];
            let entity_containers = vec![];
            let entity_types = vec![];
            let enum_types = vec![];
            let functions = vec![];
            let usings = vec![];
            let value_terms = vec![];
            Self {
                annotations,
                associations,
                complex_types,
                entity_containers,
                entity_types,
                enum_types,
                functions,
                usings,
                value_terms,
            }
        }

        pub fn new_entity(&mut self, name: &str) {
            let e = EntityType::new(name);
            self.entity_types.push(e);
        }

        pub fn new_property(&mut self, entity: &str, name: &str) {
            if let Some(e) = self.entity_types.iter_mut().find(|x| x.name == entity) {
                e.properties.push(Property::new(name));
            }
        }

        pub fn new_key(&mut self, entity: &str, key: &[&str]) {
            if let Some(e) = self.get_entity(entity) {
                e.key = Some(Key {
                    properties: key.iter().map(|x| x.to_string()).collect(),
                });
            }
        }

        fn get_entity(&mut self, entity: &str) -> Option<&mut EntityType> {
            self.entity_types.iter_mut().find(|x| x.name == entity)
        }
    }
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Annotation {}
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Association {}
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ComplexType {
        pub name: String,
        pub base: Option<String>,
        pub open: bool,
        pub properties: Vec<Property>,
    }
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EntityContianer {}
    #[derive(Debug, Serialize, Deserialize, Default)]
    pub struct EntityType {
        pub name: String,
        pub base: Option<String>,
        pub open: bool,
        pub key: Option<Key>,
        pub properties: Vec<Property>,
    }

    impl EntityType {
        pub fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
                base: None,
                open: true,
                key: None,
                properties: vec![],
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    pub struct EnumType {
        pub name: String,
        pub underlying_type: String,
        pub is_flags: bool,
        pub members: Vec<Member>,
    }
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Function {}
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Key {
        pub properties: Vec<String>,
    }
    #[derive(Debug, Serialize, Deserialize, Default)]
    pub struct Member {
        pub name: String,
        pub value: String,
    }
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Using {}
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ValueTerm {}
    #[derive(Debug, Clone, Serialize, Deserialize, Default)]
    pub struct Property {
        pub name: String,
        #[serde(rename = "type")]
        pub ptype: String,
        pub nullable: bool,
        pub navigation: bool,
    }
    impl Property {
        fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
                ptype: String::from("string"),
                nullable: true,
                navigation: false,
            }
        }
    }
}

pub trait HelloMacro {
    fn hello();
}

#[cfg(test)]
mod testing {
    use crate::{
        list::ListValue, number::Number, primitive::PrimitiveValue, structure::StructureValue,
        value::Value,
    };

    #[test]
    fn serialize_struct_to_yaml() {
        let value = Value::StructureValue(setup_typed_structure());
        let s = serde_yaml::to_string(&value).unwrap();
        let expected = r#"City: Berlin
Country@associationLink: Customers('ALFKI')/Address/Country/$ref
Country@navigationLink: Customers('ALFKI')/Address/Country
PostalCode: 12209
Region: null
Street: Obere Str. 57
"#;
        assert_eq!(expected, s);
    }

    #[test]
    fn serialize_list_to_yaml() {
        let mut result = ListValue::new();
        result.push(Value::StructureValue(setup_typed_structure()));
        result.push(Value::StructureValue(setup_typed_structure()));
        let value = Value::ListValue(result);
        let s = serde_yaml::to_string(&value).unwrap();
        let expected = r#"- City: Berlin
  Country@associationLink: Customers('ALFKI')/Address/Country/$ref
  Country@navigationLink: Customers('ALFKI')/Address/Country
  PostalCode: 12209
  Region: null
  Street: Obere Str. 57
- City: Berlin
  Country@associationLink: Customers('ALFKI')/Address/Country/$ref
  Country@navigationLink: Customers('ALFKI')/Address/Country
  PostalCode: 12209
  Region: null
  Street: Obere Str. 57
"#;
        assert_eq!(expected, s);
    }

    #[test]
    fn test_1() {
        let v = PrimitiveValue::from("xyz");
        assert_eq!("xyz", v);
    }

    #[test]
    fn test_2() {
        let v = PrimitiveValue::from(1);
        assert_eq!(v.to_string(), "1");
    }

    #[test]
    fn test_custom() {
        let v = PrimitiveValue::Custom {
            datatype: "SAPDATE".into(),
            value: "20240101".into(),
        };
        assert_eq!(v.to_string(), "20240101:SAPDATE")
    }

    #[test]
    fn test_bool() {
        let v = PrimitiveValue::Boolean(true);
        assert_eq!(v.to_string(), "true");
    }

    #[test]
    fn test_null() {
        let v = PrimitiveValue::Null;
        assert_eq!(v.to_string(), "null");
    }

    #[test]
    fn test_untyped_structure() {
        let mut v = StructureValue::new();
        v["ID"] = "ALFKI".into();
        v["CompanyName"] = "Alfreds Futterkiste".into();
        v["ContactName"] = "Maria Anders".into();
        v["ContactTitle"] = "Sales Representative".into();
        v["Phone"] = "030-0074321".into();
        v["Fax"] = "030-0076545".into();

        let mut addr = StructureValue::new();
        addr["Street"] = "Obere Str. 57".into();
        addr["City"] = "Berlin".into();
        addr["Region"] = Value::null();
        addr["PostalCode"] = "D-12209".into();
        addr["Country@associationLink"] = "Customers('ALFKI')/Address/Country/$ref".into();
        addr["Country@navigationLink"] = "Customers('ALFKI')/Address/Country".into();

        v["Address"] = addr.into();
        // "Orders@associationLink": "Customers('ALFKI')/Orders/$ref",
        // "Orders@navigationLink": "Customers('ALFKI')/Orders"
        println!("{v:#?}");
    }

    #[test]
    fn test_typed_structure() {
        let addr = setup_typed_structure();

        println!("{addr:#?}")
    }

    fn setup_typed_structure() -> StructureValue {
        let mut addr = StructureValue::new_with_type("Address");
        addr["Street"] = "Obere Str. 57".into();
        addr["City"] = "Berlin".into();
        addr["Region"] = Value::null();
        addr["PostalCode"] = 12209.into();
        addr["Country@associationLink"] = "Customers('ALFKI')/Address/Country/$ref".into();
        addr["Country@navigationLink"] = "Customers('ALFKI')/Address/Country".into();
        addr
    }

    #[test]
    fn test_list() {
        let mut lst = ListValue::new();
        lst.push("Test".into());
        lst.push(1.into());
    }

    #[test]
    fn test_mutate_list() {
        let mut lst = ListValue::new();
        for idx in 1..5 {
            let mut rec = StructureValue::new();
            rec["id"] = idx.into();
            lst.push(rec.into());
        }

        for r in lst.iter_mut() {
            match r {
                Value::StructureValue(sv) => {
                    sv["checked"] = true.into();
                }
                _ => todo!(),
            }
        }
        println!("{lst:#?}");
    }

    #[test]
    fn from_f64() {
        let v: f64 = 1298.23;
        let x = Value::from(v);
        assert_eq!("1298.23", x.to_string());
    }

    #[test]
    fn to_i64_err() {
        let v = Number::from(12.5);
        assert!(!v.is_int());
    }

    mod atom {
        use std::{cell::RefCell, rc::Rc};

        use crate::structure::StructureValue;

        #[test]
        fn format_primitive() {
            let buf = Rc::new(RefCell::new(String::new()));
            let fmt = crate::atom::Format::new(buf.clone());
            let atom: crate::value::Value = "Test".into();
            fmt.convert(&atom).unwrap();
            assert_eq!("Test", *buf.borrow());
        }

        #[test]
        fn format_struct() {
            let buf = Rc::new(RefCell::new(String::new()));
            let fmt = crate::atom::Format::new(buf.clone());
            let val: crate::value::Value = {
                let mut v = StructureValue::new_with_type("test");
                v["a"] = 1.into();
                v["asdlfkjsdf"] = "testing more values".into();
                v.into()
            };
            fmt.convert(&val).unwrap();
            assert_eq!(
                "<test><a>1</a><asdlfkjsdf>testing more values</asdlfkjsdf></test>",
                *buf.borrow()
            );
        }
    }

    mod json {
        use std::{cell::RefCell, rc::Rc};

        use crate::{list::ListValue, structure::StructureValue};

        #[test]
        fn format_primitive() {
            let buf = Rc::new(RefCell::new(String::new()));
            let fmt = crate::json::Format::new(buf.clone());
            let val: crate::value::Value = "Test".into();
            fmt.convert(&val).unwrap();
            assert_eq!("\"Test\"", *buf.borrow());
        }

        #[test]
        fn format_struct() {
            let buf = Rc::new(RefCell::new(String::new()));
            let fmt = crate::json::Format::new(buf.clone());
            let val: crate::value::Value = {
                let mut v = StructureValue::new_with_type("test");
                v["a"] = 1.into();
                v["asdlfkjsdf"] = "testing more values".into();
                v.into()
            };
            fmt.convert(&val).unwrap();
            assert_eq!(
                r#"{"__metadata":{"type":"test"},"a":"1","asdlfkjsdf":"testing more values"}"#,
                *buf.borrow()
            );
        }

        #[test]
        fn format_list() {
            let buf = Rc::new(RefCell::new(String::new()));
            let fmt = crate::json::Format::new(buf.clone());
            let val: crate::value::Value = {
                let mut v = ListValue::new();
                v.push(1.into());
                v.push("testing more values".into());
                v.into()
            };
            fmt.convert(&val).unwrap();
            assert_eq!(r#"["1","testing more values"]"#, *buf.borrow());
        }
    }
}
