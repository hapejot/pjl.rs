pub fn add(left: usize, right: usize) -> usize {
    left + right
}

use core::panic;

use itertools::Itertools;
use santiago::{grammar::Grammar, lexer::LexerRules};

#[derive(Debug, Clone)]
pub enum AttributeModel {
    Empty,
    Name(String),
    RefTo(String),
    Optional(String),
    Many0(String),
    Many1(String),
    Named {
        name: String,
        model: Box<AttributeModel>,
    },
}

pub fn lexer_rules() -> LexerRules {
    santiago::lexer_rules!(
    "DEFAULT" | "*" = string "*";
    "DEFAULT" | "+" = string "+";
    "DEFAULT" | "?" = string "?";
    "DEFAULT" | "->" = string "->";
    "DEFAULT" | "as" = string "as";
    "DEFAULT" | "WS" = pattern r"\s" => |lexer| lexer.skip();
    "DEFAULT" | "NAME" = pattern r"[a-zA-Z_-]+";
    )
}

fn produce_named(mut r: Vec<AttributeModel>) -> AttributeModel {
    assert!(r.len() > 2);
    let name = r.remove(2);
    let expr = r.remove(0);
    match name {
        AttributeModel::Name(name) => AttributeModel::Named {
            name,
            model: Box::new(expr),
        },
        _ => todo!(),
    }
}
fn produce_ref_to(mut r: Vec<AttributeModel>) -> AttributeModel {
    assert!(r.len() > 1);
    let arg = r.remove(1);
    match arg {
        AttributeModel::Name(name) => AttributeModel::RefTo(name),
        x => todo!("impl {:?}", x),
    }
}
fn produce_many0(mut r: Vec<AttributeModel>) -> AttributeModel {
    assert!(r.len() > 0);
    let arg = r.remove(0);
    match arg {
        AttributeModel::Name(name) => AttributeModel::Many0(name),
        x => todo!("impl {:?}", x),
    }
}
fn produce_many1(mut r: Vec<AttributeModel>) -> AttributeModel {
    assert!(r.len() > 0);
    let arg = r.remove(0);
    match arg {
        AttributeModel::Name(name) => AttributeModel::Many1(name),
        x => todo!("impl {:?}", x),
    }
}
pub fn grammar_rules() -> Grammar<AttributeModel> {
    santiago::grammar! {
        "model" => rules "name" ;
        "model" => rules "ref_to" "model" => produce_ref_to;
        "model" => rules "model" "many1" => produce_many1;
        "model" => rules "model" "many0" => produce_many0;
        "model" => rules "model" "as" "name" => produce_named;
        "name" => lexemes "NAME" => |l| AttributeModel::Name(l[0].raw.clone());
        "ref_to" => lexemes "->" => |_| AttributeModel::Empty;
        "many0" => lexemes "*"  => |_| AttributeModel::Empty;
        "many1" => lexemes "+"  => |_| AttributeModel::Empty;
        "optional" => lexemes "?" => |_| AttributeModel::Empty;
        "as" => lexemes "as" => |_| AttributeModel::Empty;
    }
}

fn parse(src: &str) -> AttributeModel {
    let lexing_rules = lexer_rules();
    let grammar = grammar_rules();
    let r = match santiago::lexer::lex(&lexing_rules, src) {
        Ok(lexemes) => santiago::parser::parse(&grammar, &lexemes),
        Err(err) => panic!("parse error: {err} in '{src}'"),
    };
    match r {
        Ok(r) => r.iter().nth(0).unwrap().as_abstract_syntax_tree(),
        Err(err) => panic!("parse error: {err} in '{src}'"),
    }
}

impl std::fmt::Display for AttributeModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttributeModel::Empty => write!(f, ""),
            AttributeModel::Name(name) => write!(f, "{name}"),
            AttributeModel::RefTo(name) => write!(f, "-> {name}"),
            AttributeModel::Many0(name) => write!(f, "{name}*"),
            AttributeModel::Many1(name) => write!(f, "{name}+"),
            AttributeModel::Optional(name) => write!(f, "{name}?"),
            AttributeModel::Named { name, model } => write!(f, "{} as {}", model, name),
        }
    }
}

mod file {
    use serde::{Deserialize, Serialize};
    use serde_yaml::Value;
    use std::collections::BTreeMap;
    #[derive(Serialize, Deserialize, Debug)]
    pub struct ModelFile {
        model: Model,
    }
    impl ModelFile {
        pub(crate) fn dump(&self) {
            self.model.dump();
        }

        pub(crate) fn entities(&self) -> Vec<(&String, &Entity)> {
            self.model.entities.iter().collect::<Vec<_>>()
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct Model {
        name: String,
        entities: BTreeMap<String, Entity>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub enum Entity {
        Atom { basetype: String },
        Enum { values: BTreeMap<String, Value> },
        Structure { attributes: BTreeMap<String, Value> },
    }
    impl Entity {
        pub(crate) fn attributes(&self) -> Vec<(&String, &String)> {
            match self {
                Entity::Atom { basetype } => vec![],
                Entity::Enum { values } => vec![],
                Entity::Structure { attributes } => attributes
                    .iter()
                    .map(|(n, v)| {
                        (
                            n,
                            match v {
                                Value::Null => todo!(),
                                Value::Bool(_) => todo!(),
                                Value::Number(_) => todo!(),
                                Value::String(s) => s,
                                Value::Sequence(_) => todo!(),
                                Value::Mapping(_) => todo!(),
                                Value::Tagged(_) => todo!(),
                            },
                        )
                    })
                    .collect::<Vec<_>>(),
            }
        }
    }

    impl Model {
        fn dump(&self) {
            println!("# {}", self.name);
            println!();
            for (n, e) in self.entities.iter() {
                println!();
                println!("## {}", n);
                println!();
                println!("{:#?}", e);
            }
        }
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn load_yaml() {
        let s = std::fs::read_to_string("sample-database.xdd").unwrap();
        let v = serde_yaml::from_str::<file::ModelFile>(s.as_str()).unwrap();
        for (n, e) in v.entities() {
            println!("entity: {n}");
            for (an, src) in e.attributes() {
                println!("{an:?}: {:}", parse(src));
            }
        }
    }

    // AttributeModel::Empty => write!(f, ""),
    // AttributeModel::Name(name) => write!(f, "{name}"),
    #[test]
    fn convert_name() {
        let src = "name";
        check(src);
        match parse(src) {
            AttributeModel::Name(n) => assert_eq!(n, "name"),
            _ => self::panic!(),
        }
    }
    // AttributeModel::RefTo(name) => write!(f, "-> {name}"),
    #[test]
    fn convert_ref_to() {
        let src = "-> Entity";
        check(src);
        match parse(src) {
            AttributeModel::RefTo(n) => assert_eq!(n, "Entity"),
            _ => self::panic!(),
        }
    }

    fn check(src: &str) {
        assert_eq!(src, format!("{}", parse(src)));
    }
    // AttributeModel::Many0(name) => write!(f, "{name}*"),
    #[test]
    fn convert_many0() {
        let src = "Entity*";
        check(src);
        match parse(src) {
            AttributeModel::Many0(n) => assert_eq!(n, "Entity"),
            _ => self::panic!(),
        }
    }
    // AttributeModel::Many1(name) => write!(f, "{name}+"),
    #[test]
    fn convert_many1() {
        let src = "Entity+";
        check(src);
        match parse(src) {
            AttributeModel::Many1(n) => assert_eq!(n, "Entity"),
            _ => self::panic!(),
        }
    }
    // AttributeModel::Named { name, model } => write!(f, "{} as {}", model, name),
    #[test]
    fn convert_named() {
        let src = "Entity* as something";
        check(src);
        match parse(src) {
            AttributeModel::Named { name, .. } => assert_eq!(name, "something"),
            _ => self::panic!(),
        }
    }

    #[test]
    fn convert_optional() {
        let src = "Entity?";
        check(src);
        match parse(src) {
            AttributeModel::Optional(name) => assert_eq!(name, "Entity"),
            _ => self::panic!(),
        }
    }
}
