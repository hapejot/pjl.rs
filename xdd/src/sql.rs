use crate::*;
use itertools::Itertools;
use std::collections::BTreeMap as Dict;

#[derive(Debug)]
pub struct SqlTable {
    name: String,
    fields: Vec<SqlField>,
}

impl SqlTable {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            fields: vec![],
        }
    }

    #[allow(dead_code)]
    fn add_field(&mut self, an: &str, types: String, null: bool) {
        let f = SqlField {
            name: an.into(),
            types,
            null,
            key: None,
        };
        self.fields.push(f);
    }

    #[allow(dead_code)]
    fn add_key(&mut self, an: &str, types: String) {
        let f = SqlField {
            name: an.into(),
            types,
            null: false,
            key: Some(1),
        };
        self.fields.push(f);
    }
    #[allow(dead_code)]
    fn remove(&mut self, arg: &str) {
        if let Some((x, _)) = self.fields.iter().find_position(|x| x.name == arg) {
            self.fields.remove(x);
        }
    }
}

#[derive(Debug)]
pub struct SqlField {
    name: String,
    types: String,
    null: bool,
    key: Option<usize>,
}

#[derive(Debug)]
pub struct SqlModel {
    tables: Dict<String, SqlTable>,
}

impl SqlModel {
    fn new() -> Self {
        let tables = Dict::new();
        Self { tables }
    }

    pub fn ensure_table(&mut self, name: &str) -> &mut SqlTable {
        let ts = &mut self.tables;
        if !ts.contains_key(name) {
            ts.insert(name.into(), SqlTable::new(name));
        }
        ts.get_mut(name).unwrap()
    }

    #[allow(dead_code)]
    pub fn get_table(&mut self, name: &str) -> Option<&mut SqlTable> {
        let ts = &mut self.tables;
        ts.get_mut(name)
    }

    fn generate_table(&mut self, v: &ModelFile, name: &String, entity: &StructureEntity) {
        let t = self.ensure_table(entity.sql_name().as_str());
        for (an, src) in entity.attributes() {
            eprintln!("{:} {} - {:?}", entity.sql_name(), an, src);
            let f = make_field_def(&src, v, an);
            t.fields.push(f);
        }
    }
}

fn make_field_def(src: &AttributeModel, v: &ModelFile, an: &String) -> SqlField {
    let mut types = String::new();
    let mut null = false;
    let mut key = false;
    match src {
        AttributeModel::Empty => todo!("why is field {} empty?", an),
        AttributeModel::Name(type_name) => {
            let types = v.derive_type_name(&src).clone();
            SqlField {
                name: an.clone(),
                types,
                null: false,
                key: None,
            }
        }
        AttributeModel::RefTo(_name, _) => SqlField {
            name: an.clone(),
            types: String::from("string"),
            null: false,
            key: None,
        },
        AttributeModel::Optional(model) => {
            let mut f = make_field_def(model, v, an);
            f.null = true;
            f
        }
        AttributeModel::Key(model) => {
            let mut f = make_field_def(model, v, an);
            f.key = Some(1);
            f
        }
        _ => todo!("{:?}", src),
    }
}

impl std::fmt::Display for SqlModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (_, t) in self.tables.iter() {
            writeln!(f, "{};", t)?;
        }
        Ok(())
    }
}
impl std::fmt::Display for SqlTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "CREATE TABLE {} (", self.name)?;
        let mut first = true;
        let mut primary_key = vec![];
        for x in self.fields.iter() {
            if first {
                first = false;
            } else {
                writeln!(f, ",")?;
            }
            write!(f, "    {} {}", x.name, x.types)?;
            if !x.null {
                write!(f, " NOT NULL")?;
            }
            if x.key.is_some() {
                primary_key.push(x.name.as_str());
            }
        }
        if primary_key.len() > 0 {
            writeln!(f, ",")?;
            write!(f, "    primary key ({}) ", primary_key.join(","))?;
        }
        writeln!(f, ")")?;
        Ok(())
    }
}

fn handle_attribute(
    src: AttributeModel,
    v: &ModelFile,
    t: &mut SqlTable,
    an: &String,
    ename: &String,
    backlog: &mut Vec<(String, String)>,
) {
    match src {
        AttributeModel::Empty => todo!(),
        AttributeModel::Name(type_name) => {
            let att_type = v.resolve_type(&type_name);
            t.add_field(an, att_type, false);
        }
        AttributeModel::RefTo(_name, _) => {
            t.add_field(an, "string".into(), false);
        }
        AttributeModel::Optional(type_name) => {
            t.add_field(an, v.derive_type_name(&type_name).clone(), true);
        }
        AttributeModel::Key(model) => todo!(),
        AttributeModel::Many0(_) => self::panic!(),
        AttributeModel::Many1(_) => self::panic!(),
        AttributeModel::Named { name, model } => match *model {
            AttributeModel::Many0(x) => {
                if let (Some(n1), Some(n2)) =
                    (v.resolve_name1(ename.as_str()), v.resolve_name2(x.as_str()))
                {
                    backlog.push((name.clone(), n1));
                    backlog.push((name, n2));
                }
                else {
                    eprintln!("{} * {} could not be resolved.", ename, x);
                }
            }
            AttributeModel::Many1(x) => {
                if let (Some(n1), Some(n2)) =
                    (v.resolve_name1(ename.as_str()), v.resolve_name2(x.as_str()))
                {
                    backlog.push((name.clone(), n1));
                    backlog.push((name, n2));
                }
                else {
                    eprintln!("{} * {} could not be resolved.", ename, x);
                }
            }
            _ => todo!(),
        },
    };
}

pub fn generate_sql(v: &ModelFile) -> SqlModel {
    let mut model = SqlModel::new();
    // let mut backlog = vec![];
    for (n, e) in v.entities() {
        match e {
            crate::Entity::Structure(entity) => {
                model.generate_table(v, n, entity);
            }
            _ => {}
        };
    }
    // loop {
    //     if let Some(t) = backlog.pop() {
    //         let _tab = model.ensure_table(t.as_str());
    //     } else {
    //         break;
    //     }
    // }
    model
}

pub fn diff(expected: &str, actual: &str) -> bool {
    let mut expected_lines = expected.split('\n').collect::<Vec<_>>();
    let mut actual_lines = actual.split('\n').collect::<Vec<_>>();
    let mut result = true;
    let mut idx = 0;
    while expected_lines.len() > 0 && actual_lines.len() > 0 {
        let expected_line = expected_lines.remove(0);
        let actual_line = actual_lines.remove(0);
        idx += 1;
        if expected_line != actual_line {
            eprintln!("actual({}): '{}'", idx, actual_line);
            eprintln!("expected:   '{}'", expected_line);
            result = false;
            break;
        }
    }
    result
}
