use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::collections::BTreeMap;

use crate::AttributeModel;
#[derive(Serialize, Deserialize, Debug)]
pub struct ModelFile {
    model: Model,
}
#[allow(dead_code)]
impl ModelFile {
    pub fn from_str(src: &str) -> ModelFile {
        let mut v = serde_yaml::from_str::<Self>(src).unwrap();
        v.initial_cleanup();
        v
    }

    pub(crate) fn dump(&self) {
        self.model.dump();
    }

    pub fn entities(&self) -> Vec<(&String, &Entity)> {
        self.model.entities.iter().collect::<Vec<_>>()
    }

    pub fn resolve_type(&self, type_name: &str) -> String {
        match self.model.entities.get(type_name) {
            Some(x) => match x {
                Entity::Atom { basetype } => basetype.clone(),
                Entity::Enum { .. } => String::from("string"),
                Entity::Structure { .. } => todo!(),
            },
            None => String::from(type_name),
        }
    }

    fn initial_cleanup(&mut self) {
        for (entity_name, entity) in self.model.entities.iter_mut() {
            match entity {
                Entity::Atom { .. } => {}
                Entity::Enum { .. } => {}
                Entity::Structure(StructureEntity { name, sql_name, .. }) => {
                    *name = Some(entity_name.clone());
                    *sql_name = Some(entity_name.to_lowercase());
                }
            }
        }
    }

    pub fn resolve_name1(&self, entity_name: &str) -> Option<String> {
        if let Some(e) = self.lookup_entity(entity_name) {
            let name = e.sql_name();
            Some(format!("{name}_id"))
        } else {
            None
        }
    }

    pub fn resolve_name2(&self, entity_name: &str) -> Option<String> {
        if let Some(e) = self.lookup_entity(entity_name) {
            let name = e.sql_name();
            Some(format!("{name}_id"))
        } else {
            None
        }
    }

    fn lookup_entity(&self, entity_name: &str) -> Option<&Entity> {
        let m = &self.model;
        if m.entities.contains_key(entity_name) {
            let res = &self.model.entities[entity_name];
            Some(res)
        } else {
            None
        }
    }

    pub fn derive_type_name(&self, model: &AttributeModel) -> String {
        match model {
            AttributeModel::Empty => todo!(),
            AttributeModel::Name(name) => match self.lookup_entity(name) {
                Some(Entity::Atom { basetype }) => basetype.clone(),
                Some(Entity::Enum { values: _ }) => String::from("string"),
                Some(Entity::Structure(StructureEntity {
                    name,
                    sql_name: _,
                    attributes: _,
                })) => name.as_ref().unwrap().clone(),
                None => name.clone(),
            },
            AttributeModel::RefTo(_, _) => todo!(),
            AttributeModel::Optional(_) => todo!(),
            AttributeModel::Many0(_) => todo!(),
            AttributeModel::Many1(_) => todo!(),
            AttributeModel::Key(_) => todo!(),
            AttributeModel::Named { name: _, model: _ } => todo!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Model {
    name: String,
    entities: BTreeMap<String, Entity>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StructureEntity {
    name: Option<String>,
    sql_name: Option<String>,
    attributes: BTreeMap<String, Value>,
}
impl StructureEntity {
    pub fn sql_name(&self) -> String {
        let result = match self.sql_name.as_ref() {
            Some(name) => name.clone(),
            None => match self.name.as_ref() {
                Some(name) => name.to_lowercase(),
                None => String::from("anonymous_table"),
            },
        };
        result
    }
    pub fn attributes(&self) -> Vec<(&String, AttributeModel)> {
        self.attributes
            .iter()
            .map(|(n, v)| {
                (
                    n,
                    match v {
                        Value::Null => todo!(),
                        Value::Bool(_) => todo!(),
                        Value::Number(_) => todo!(),
                        Value::String(s) => match crate::parse(s) {
                            Ok(attr) => attr,
                            Err(err) => {
                                println!("{n} {}", err);
                                AttributeModel::Empty
                            }
                        },
                        Value::Sequence(_) => todo!(),
                        Value::Mapping(_) => todo!(),
                        Value::Tagged(_) => todo!(),
                    },
                )
            })
            .collect::<Vec<_>>()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Entity {
    Atom { basetype: String },
    Enum { values: BTreeMap<String, Value> },
    Structure(StructureEntity),
}
#[allow(dead_code)]
impl Entity {
    pub fn sql_name(&self) -> &str {
        match self {
            Entity::Atom { basetype } => basetype.as_str(),
            Entity::Enum { .. } => "string",
            Entity::Structure(StructureEntity {
                sql_name: Some(sql_name),
                ..
            }) => sql_name.as_str(),
            Entity::Structure(StructureEntity {
                name: Some(name), ..
            }) => name.as_str(),
            _ => todo!(),
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
