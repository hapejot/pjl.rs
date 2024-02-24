pub mod atom;
pub mod json;
pub mod list;
pub mod number;
pub mod primitive;
pub mod structure;
pub mod value;

#[cfg(test)]
mod testing {
    use crate::{
        list::ListValue, primitive::PrimitiveValue, structure::StructureValue, value::Value,
    };

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
        let mut addr = StructureValue::new_with_type("Address");
        addr["Street"] = "Obere Str. 57".into();
        addr["City"] = "Berlin".into();
        addr["Region"] = Value::null();
        addr["PostalCode"] = 12209.into();
        addr["Country@associationLink"] = "Customers('ALFKI')/Address/Country/$ref".into();
        addr["Country@navigationLink"] = "Customers('ALFKI')/Address/Country".into();

        println!("{addr:#?}")
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
        let v:f64 = 1298.23;
        let x = Value::from(v);
        assert_eq!("1298.23", x.to_string());
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
            assert_eq!("<test><a>1</a><asdlfkjsdf>testing more values</asdlfkjsdf></test>", *buf.borrow());
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
            assert_eq!(r#"{"__metadata":{"type":"test"},"a":"1","asdlfkjsdf":"testing more values"}"#, *buf.borrow());
        }

        #[test]
        fn format_list() {
            let buf = Rc::new(RefCell::new(String::new()));
            let fmt = crate::json::Format::new(buf.clone());
            let val: crate::value::Value = {
                let mut v = ListValue::new();
                v.push( 1.into());
                v.push("testing more values".into());
                v.into()
            };
            fmt.convert(&val).unwrap();
            assert_eq!(r#"["1","testing more values"]"#, *buf.borrow());
        }
    }

}
