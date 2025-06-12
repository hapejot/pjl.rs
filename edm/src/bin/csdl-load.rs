use edm::csdl::{ComplexType, EntityType, EnumType, Key, Member, Property, Schema};
use serde::{Deserialize, Serialize};
use xml_derive::HelloMacro;
// use xml_derive::HelloMacro;
use edm::HelloMacro;
use paste::paste;
use std::{fs::File, io::Read, ptr::null};
use xml::{name::OwnedName, reader::XmlEvent, EventReader};

macro_rules! xml1 {
    ($self:expr => $namespace:expr , $lname:expr  => $action:expr ) => {
        match $self.current.as_ref() {
            Some(XmlEvent::StartElement {
                name:
                    OwnedName {
                        local_name,
                        namespace: Some(ns),
                        ..
                    },
                ..
            }) => {
                if local_name == $lname && ns == $namespace {
                    $self.next_event();
                    $action
                } else {
                    todo!("{}", local_name);
                }
            }
            _ => todo!(),
        }
    };
}

macro_rules! xml2 {
    (doctype $doctype:ident $(element $rule:ident ( $($v:expr),* ) )* ) => {
        paste!{
            mod $doctype {
                pub mod s {
                    #[allow(non_snake_case)]
                    #[allow(dead_code)]
                                        pub type PCDATA = String;
                $(
                    pub struct $rule {
                    $(
                        #[allow(non_snake_case)]
                        #[allow(dead_code)]
                                                pub [<m_ $v>]  :Option<$v>,
                    )*
                    }
                )*
            }
            #[allow(non_snake_case)]
            #[allow(dead_code)]
            pub fn PCDATA() {}



            $(
                pub fn $rule() {
                $(
                    $v;
                )*
                }
            )*
        }
    }
    }
}

xml2!(
    doctype note
    element note (to,from,heading,body)
    element to (PCDATA)
    element from (PCDATA)
    element heading (PCDATA)
    element body (PCDATA)
);

const NS_EDMX: &str = "http://docs.oasis-open.org/odata/ns/edmx";
const NS_EDM: &str = "http://docs.oasis-open.org/odata/ns/edm";
const EDMX: &str = "Edmx";
const DATA_SERVICES: &str = "DataServices";
const SCHEMA: &str = "Schema";
const ENUM_TYPE: &str = "EnumType";
const MEMBER: &str = "Member";
const COMPLEX_TYPE: &str = "ComplexType";
const ENTITY_TYPE: &str = "EntityType";
const PROPERTY: &str = "Property";
const NAVIGATION_PROPERTY: &str = "NavigationProperty";

struct App<'a> {
    events: EventReader<&'a [u8]>,
    current: Option<XmlEvent>,
}

impl<'a> App<'a> {
    fn next_event(&mut self) {
        self.current = match self.events.next() {
            Ok(e) => Some(e),
            Err(_) => None,
        }
    }
    fn read_start(&mut self) {
        match self.current.as_ref() {
            Some(XmlEvent::StartDocument { .. }) => {
                println!("start");
                self.next_event();
            }
            _ => {}
        }
    }

    fn doc(&mut self) {
        self.read_start();
        self.skip_whitespace();
        match self.current.as_ref() {
            Some(XmlEvent::StartElement {
                name:
                    OwnedName {
                        local_name,
                        namespace: Some(ns),
                        ..
                    },
                ..
            }) => {
                if local_name == EDMX && ns == NS_EDMX {
                    self.next_event();
                    self.edmx();
                } else {
                    todo!("{}", local_name);
                }
            }
            _ => todo!(),
        }
    }

    fn edmx(&mut self) {
        self.skip_whitespace();
        xml1!(self => NS_EDMX, DATA_SERVICES => self.data_service());
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.current.as_ref() {
                Some(XmlEvent::Whitespace(_)) => self.next_event(),
                _ => break,
            }
        }
    }

    fn expect_end_element(&mut self, expected_name: &str) {
        loop {
            match self.current.as_ref() {
                Some(XmlEvent::EndElement { name }) => {
                    if name.local_name == expected_name {
                        self.next_event();
                        break;
                    }
                }
                None => break,
                Some(_) => {}
            }
            self.next_event();
        }
    }

    fn schema(&mut self) {
        let mut result = Schema {
            annotations: vec![],
            associations: vec![],
            complex_types: vec![],
            entity_containers: vec![],
            entity_types: vec![],
            enum_types: vec![],
            functions: vec![],
            usings: vec![],
            value_terms: vec![],
        };
        loop {
            self.skip_whitespace();
            match self.current.as_ref() {
                Some(XmlEvent::StartElement {
                    name:
                        OwnedName {
                            local_name,
                            namespace: Some(namespace),
                            ..
                        },
                    ..
                }) => {
                    if namespace == NS_EDM {
                        match local_name.as_str() {
                            ENUM_TYPE => {
                                result.enum_types.push(self.enum_type());
                            }
                            COMPLEX_TYPE => {
                                result.complex_types.push(self.complex_type());
                            }
                            ENTITY_TYPE => {
                                result.entity_types.push(self.entity_type());
                            }
                            _ => {
                                let name = local_name.clone();
                                self.expect_end_element(name.as_str());
                            }
                        }
                        self.next_event();
                    }
                }
                Some(XmlEvent::EndElement { name }) => {
                    if name.local_name == SCHEMA {
                        self.next_event();
                        break;
                    }
                }
                e => println!("schema element: {:?}", e),
            }
        }
        let out = serde_yaml::to_string(&result).unwrap();
        println!("{}", out);
    }

    fn data_service(&mut self) {
        self.skip_whitespace();
        match self.current.as_ref() {
            Some(XmlEvent::StartElement {
                name:
                    OwnedName {
                        local_name,
                        namespace: Some(namespace),
                        ..
                    },
                attributes,
                ..
            }) => {
                if local_name == SCHEMA && namespace == NS_EDM {
                    println!("Schema {:#?}", attributes);
                    self.next_event();
                    self.schema();
                } else {
                    todo!("{:?}", local_name)
                }
            }
            _ => todo!(),
        }
    }

    fn enum_type(&mut self) -> EnumType {
        let mut enum_name = String::from("NoName");
        match self.current.as_ref() {
            Some(XmlEvent::StartElement { attributes, .. }) => {
                for a in attributes {
                    if a.name.local_name == "Name" {
                        enum_name = a.value.clone()
                    }
                }
            }
            _ => todo!(),
        }
        let mut ret = EnumType {
            name: enum_name,
            members: vec![],
            is_flags: false,
            underlying_type: String::new(),
        };
        self.next_event();
        loop {
            self.skip_whitespace();

            match self.current.as_ref() {
                Some(XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                }) => {
                    if name.local_name == MEMBER {
                        let mut name = String::new();
                        let mut value = String::new();
                        for a in attributes {
                            if a.name.local_name == "Name" {
                                name = a.value.clone();
                            }
                            if a.name.local_name == "Value" {
                                value = a.value.clone();
                            }
                        }
                        ret.members.push(Member { name, value });
                        self.expect_end_element(MEMBER);
                    } else {
                        todo!();
                    }
                }
                _ => break,
            }
        }
        self.expect_end_element(ENUM_TYPE);
        ret
    }

    fn complex_type(&mut self) -> ComplexType {
        let mut type_name = String::from("none");
        let mut open = false;
        let mut base = None;
        match self.current.as_ref() {
            Some(XmlEvent::StartElement { attributes, .. }) => {
                for a in attributes {
                    if a.name.local_name == "Name" {
                        type_name = a.value.clone();
                    }
                    if a.name.local_name == "BaseType" {
                        base = Some(a.value.clone());
                    }
                    if a.name.local_name == "OpenType" && a.value == "true" {
                        open = true;
                    }
                }
            }
            _ => todo!(),
        }
        self.next_event();
        let properties = self.properties();
        self.expect_end_element(COMPLEX_TYPE);
        ComplexType {
            base,
            open,
            name: type_name,
            properties,
        }
    }

    fn entity_type(&mut self) -> EntityType {
        let mut name = String::from("none");
        let mut base = None;
        let mut open = false;
        match self.current.as_ref() {
            Some(XmlEvent::StartElement { attributes, .. }) => {
                for a in attributes {
                    if a.name.local_name == "Name" {
                        name = a.value.clone();
                    }
                    if a.name.local_name == "BaseType" {
                        base = Some(a.value.clone());
                    }
                    if a.name.local_name == "OpenType" && a.value == "true" {
                        open = true;
                    }
                }
            }
            _ => todo!(),
        }
        self.next_event();
        let mut key = None;
        if base == None {
            key = Some(self.key());
        }
        let properties = self.properties();
        self.expect_end_element(ENTITY_TYPE);
        EntityType {
            base,
            open,
            name,
            key,
            properties,
        }
    }

    fn properties(&mut self) -> Vec<Property> {
        let mut result = vec![];
        loop {
            match self.current.as_ref() {
                Some(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if name.local_name == PROPERTY {
                        result.push(make_property(attributes, false));
                        self.expect_end_element(PROPERTY);
                    } else if name.local_name == NAVIGATION_PROPERTY {
                        result.push(make_property(attributes, true));
                        self.expect_end_element(NAVIGATION_PROPERTY);
                    } else {
                        break;
                    }
                }
                Some(XmlEvent::Whitespace(_)) => {
                    self.next_event();
                }
                _ => break,
            }
        }
        result
    }

    fn key(&mut self) -> Key {
        let mut result = Key { properties: vec![] };
        self.skip_whitespace();
        match self.current.as_ref() {
            Some(XmlEvent::StartElement { name, .. }) => {
                if name.local_name != "Key" {
                    println!("expected Key instead of {}", name);
                    return result;
                }
                self.next_event();
                loop {
                    self.skip_whitespace();
                    match self.current.as_ref() {
                        Some(XmlEvent::StartElement {
                            name, attributes, ..
                        }) => {
                            if name.local_name == "PropertyRef" {
                                for a in attributes {
                                    if a.name.local_name == "Name" {
                                        result.properties.push(a.value.clone());
                                    }
                                }
                                self.next_event();
                            } else {
                                todo!("{}", name.local_name);
                            }
                        }
                        _ => {
                            break;
                        }
                    }
                }
            }
            _ => todo!(),
        }
        self.expect_end_element("Key");
        result
    }
}

fn make_property(attributes: &Vec<xml::attribute::OwnedAttribute>, navigation: bool) -> Property {
    let mut name = String::new();
    let mut ptype = String::new();
    let mut nullable = true;
    for a in attributes {
        if a.name.local_name == "Name" {
            name = a.value.clone();
        }
        if a.name.local_name == "Type" {
            ptype = a.value.clone();
        }
        if a.name.local_name == "Nullable" && a.value == "false" {
            nullable = false;
        }
    }

    Property {
        name,
        ptype,
        nullable,
        navigation,
    }
}

fn main() {
    // note::note();
    // let n = note::s::note{ m_to: None, m_from: None, m_heading: None, m_body: None };
    let mut f = File::open("metadata.xml").unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();

    let mut app = App {
        events: EventReader::from_str(buf.as_str()),
        current: None,
    };
    app.next_event();

    app.doc();
}
