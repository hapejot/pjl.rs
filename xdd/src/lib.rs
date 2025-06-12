use core::panic;
use santiago::{grammar::Grammar, lexer::LexerRules};

pub use file::{Entity, ModelFile, StructureEntity};
pub use sql::{diff, generate_sql, SqlField, SqlModel, SqlTable};

mod file;
mod sql;

#[derive(Debug, Clone)]
pub enum AttributeModel {
    Empty,
    Name(String),
    RefTo(String, Option<String>),
    Optional(Box<AttributeModel>),
    Many0(String),
    Many1(String),
    Named {
        name: String,
        model: Box<AttributeModel>,
    },
    Key(Box<AttributeModel>),
}

pub fn lexer_rules() -> LexerRules {
    santiago::lexer_rules!(
    "DEFAULT" | "*" = string "*";
    "DEFAULT" | "(" = string "(";
    "DEFAULT" | ")" = string ")";
    "DEFAULT" | "+" = string "+";
    "DEFAULT" | "?" = string "?";
    "DEFAULT" | "->" = string "->";
    "DEFAULT" | "as" = string "as";
    "DEFAULT" | "WS" = pattern r"\s" => |lexer| lexer.skip();
    "DEFAULT" | "NAME" = pattern r"[a-zA-Z0-9_-]+";
    "DEFAULT" | "REMARK" = pattern r"#.*$";
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
    let _arrow = r.remove(0);
    let model = r.remove(0);
    model
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
fn produce_optional(mut r: Vec<AttributeModel>) -> AttributeModel {
    assert!(r.len() > 0);
    let arg = r.remove(0);
    match arg {
        AttributeModel::Empty => panic!(),
        model => AttributeModel::Optional(Box::new(model)),
    }
}
fn produce_target(mut r: Vec<AttributeModel>) -> AttributeModel {
    let entity = r.remove(0);
    let _open = if r.len() > 0 {
        r.remove(0)
    } else {
        AttributeModel::Empty
    };
    let field = if r.len() > 0 { Some(r.remove(0)) } else { None };
    match (entity, field) {
        (AttributeModel::Name(name), Some(AttributeModel::Name(field))) => {
            AttributeModel::RefTo(name, Some(field))
        }
        (AttributeModel::Name(name), None) => AttributeModel::RefTo(name, None),
        x => todo!("impl {:?}", x),
    }
}
fn produce_primary_key(mut r: Vec<AttributeModel>) -> AttributeModel {
    let _plus = r.remove(0);
    let model = r.remove(0);
    AttributeModel::Key(Box::new(model))
}

fn produce_spec(mut r: Vec<AttributeModel>) -> AttributeModel {
    match r.len() {
        1 => r.remove(0),
        2 => r.remove(0),
        _ => panic!("unexpected number of params"),
    }
}

pub fn grammar_rules() -> Grammar<AttributeModel> {
    santiago::grammar! {
        "spec" => rules "model" => produce_spec;
        "spec" => rules "model" "remark" => produce_spec;
        "model0" => rules "name" ;
        "model0" => rules "ref_to" "target" => produce_ref_to;
        "model" => rules "model0";
        "model" => rules "model0" "option" => produce_optional;
        "model" => rules "plus" "model0" => produce_primary_key;
        "model" => rules "plus" "model0" "option"=> produce_primary_key;
        "model" => rules "model" "plus" => produce_many1;
        "model" => rules "model" "many0" => produce_many0;
        "model" => rules "model" "as" "name" => produce_named;
        "model" => rules "name" "option" => produce_optional;
        "target" => rules "name" => produce_target;
        "target" => rules "name" "open" "name" "close" => produce_target;
        "name" => lexemes "NAME" => |l| AttributeModel::Name(l[0].raw.clone());
        "ref_to" => lexemes "->" => |_| AttributeModel::Empty;
        "many0" => lexemes "*"  => |_| AttributeModel::Empty;
        "plus" => lexemes "+"  => |_| AttributeModel::Empty;
        "option" => lexemes "?" => |_| AttributeModel::Empty;
        "as" => lexemes "as" => |_| AttributeModel::Empty;
        "open" => lexemes "(" => |_| AttributeModel::Empty;
        "close" => lexemes ")" => |_| AttributeModel::Empty;
        "remark" => lexemes "REMARK" => |_| AttributeModel::Empty;
    }
}

#[allow(dead_code)]
fn parse(src: &str) -> Result<AttributeModel, Error> {
    let lexing_rules = lexer_rules();
    let grammar = grammar_rules();
    let r = match santiago::lexer::lex(&lexing_rules, src) {
        Ok(lexemes) => santiago::parser::parse(&grammar, &lexemes),
        Err(err) => panic!("parse error: {err} in '{src}'"),
    };
    match r {
        Ok(r) => Ok(r.iter().nth(0).unwrap().as_abstract_syntax_tree()),
        Err(err) => Err(Error::ParseError {
            entity: None,
            field: None,
            message: format!("parse error: {err} in '{src}'"),
        }),
    }
}

#[derive(Debug)]
pub enum Error {
    ParseError {
        entity: Option<String>,
        field: Option<String>,
        message: String,
    },
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::fmt::Display for AttributeModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttributeModel::Empty => write!(f, ""),
            AttributeModel::Key(model) => write!(f, "+{model}"),
            AttributeModel::Name(name) => write!(f, "{name}"),
            AttributeModel::RefTo(name, None) => write!(f, "-> {name}"),
            AttributeModel::RefTo(name, Some(field)) => write!(f, "-> {name}({field})"),
            AttributeModel::Many0(name) => write!(f, "{name}*"),
            AttributeModel::Many1(name) => write!(f, "{name}+"),
            AttributeModel::Optional(name) => write!(f, "{name}?"),
            AttributeModel::Named { name, model } => write!(f, "{} as {}", model, name),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const SAMPLE: &str = r#"
model: 
    name: actor    
    entities: 
        Date: !Atom
            basetype: string
        Gender: !Enum
            values:
                1: Male
                2: Female
        Path: !Structure
            attributes:
                name: string                
        File: !Structure
            attributes:
                pathid: -> Path
                size: int?
                name: string
        Actor:  !Structure
            attributes:
                name: string
                bio: string?
                birthPlace: string?
                birthday: Date?
                gender: Gender?
                height: string?
                lastSceneReleaseDate: string?
                measurements: string?
                rank: string?
                weight: string?
        Tag: !Structure
            attributes:
                name: string
                category: string?
                categoryOrder: string?
        Scene: !Structure
            attributes:
                title: string
                description: string?
                video_1080p_sizeBytes: string?
                video_1080p_download: string?
                video_1080p_view: string?
                video_1080p_format: string?
                video_length: int?
                actors: Actor+ as actor_scene
                tags: Tag* as scene_tag
                paths: Path* as path_scene
    "#;

    #[test]
    fn load_yaml() {
        // let s = std::fs::read_to_string("sample-database.xdd").unwrap();
        let v = ModelFile::from_str(SAMPLE);
        for (n, _e) in v.entities() {
            println!("entity: {n}");
            // for (an, src) in e.attributes() {
            //     println!("{an}: {src}");
            // }
        }
    }

    // AttributeModel::Empty => write!(f, ""),
    // AttributeModel::Name(name) => write!(f, "{name}"),
    #[test]
    fn convert_name() {
        let src = "name";
        check(src);
        match parse(src) {
            Ok(AttributeModel::Name(n)) => assert_eq!(n, "name"),
            _ => self::panic!(),
        }
    }
    // AttributeModel::RefTo(name) => write!(f, "-> {name}"),
    #[test]
    fn convert_ref_to() {
        let src = "-> Entity";
        check(src);
        match parse(src) {
            Ok(AttributeModel::RefTo(n, None)) => assert_eq!(n, "Entity"),
            _ => self::panic!(),
        }
    }

    fn check(src: &str) {
        match parse(src) {
            Ok(model) => assert_eq!(src, format!("{}", model)),
            Err(x) => self::panic!("{} could not be parsed: {}", src, x),
        }
    }

    // AttributeModel::Many0(name) => write!(f, "{name}*"),
    #[test]
    fn convert_many0() {
        let src = "Entity*";
        check(src);
        match parse(src) {
            Ok(AttributeModel::Many0(n)) => assert_eq!(n, "Entity"),
            _ => self::panic!(),
        }
    }
    // AttributeModel::Many1(name) => write!(f, "{name}+"),
    #[test]
    fn convert_many1() {
        let src = "Entity+";
        check(src);
        match parse(src) {
            Ok(AttributeModel::Many1(n)) => assert_eq!(n, "Entity"),
            _ => self::panic!(),
        }
    }
    // AttributeModel::Named { name, model } => write!(f, "{} as {}", model, name),
    #[test]
    fn convert_named() {
        let src = "Entity* as something";
        check(src);
        match parse(src) {
            Ok(AttributeModel::Named { name, .. }) => assert_eq!(name, "something"),
            _ => self::panic!(),
        }
    }

    #[test]
    fn convert_optional() {
        let src = "Entity?";
        check(src);
        match parse(src) {
            Ok(AttributeModel::Optional(x)) => match *x {
                AttributeModel::Name(name) => assert_eq!(name, "Entity"),
                _ => self::panic!(),
            },
            _ => self::panic!(),
        }
    }

    #[test]
    fn convert_reference_with_field() {
        let src = "-> Entity(fieldname)";
        check(src);
        match parse(src) {
            Ok(AttributeModel::RefTo(name, Some(field))) => {
                assert_eq!("Entity", name);
                assert_eq!("fieldname", field);
            }
            x => self::panic!("{:#?}", x),
        }
    }

    #[test]
    fn convert_opt_ref() {
        let src = "-> Entity(fieldname)?";
        check(src);
        match parse(src) {
            Ok(AttributeModel::Optional(x)) => match *x {
                AttributeModel::RefTo(name, Some(field)) => {
                    assert_eq!("Entity", name);
                    assert_eq!("fieldname", field);
                }
                _ => self::panic!(),
            },
            x => self::panic!("{:#?}", x),
        }
    }

    #[test]
    fn convert_primary_key() {
        let src = "+type";
        check(src);
        match parse(src) {
            Ok(AttributeModel::Key(x)) => match *x {
                AttributeModel::Name(name) => {
                    assert_eq!("type", name);
                }
                x => self::panic!("{:#?}", x),
            },
            x => self::panic!("{:#?}", x),
        }
    }

    #[test]
    fn convert_key_link() {
        let src = "+-> Entity(field)";
        check(src);
        match parse(src) {
            Ok(AttributeModel::Key(x)) => match *x {
                AttributeModel::RefTo(name, Some(field)) => {
                    assert_eq!("Entity", name);
                    assert_eq!("field", field);
                }
                x => self::panic!("{:#?}", x),
            },
            x => self::panic!("{:#?}", x),
        }
    }

    #[test]
    fn convert_real_world_1() {
        let src = "+zbc_step?                                 # Step number of process";
        match parse(src) {
            Ok(AttributeModel::Key(x)) => match *x {
                AttributeModel::Name(name) => {
                    assert_eq!("zbc_step", name);
                }
                x => self::panic!("{:#?}", x),
            },
            x => self::panic!("{:#?}", x),
        }
    }
}
