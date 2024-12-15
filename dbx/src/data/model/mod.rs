use log::{debug, error, info, trace};

pub mod meta {

    #[derive(Debug, Clone)]
    pub struct Relation {
        pub id: String,
        pub from: String,
        pub to: String,
        pub name: String,
        pub kind: RelationKind,
        pub fields: Vec<(String, String)>,
    }

    #[derive(Debug, Clone)]
    pub enum RelationKind {
        One,
        Many,
        ManyMany(String),
    }
    #[derive(Debug, Clone)]
    pub struct Meta {
        relations: Vec<Relation>,
    }

    impl Meta {
        pub fn new() -> Self {
            Self { relations: vec![] }
        }

        pub fn define_relation(
            &mut self,
            kind: RelationKind,
            from: &str,
            name: &str,
            to: &str,
        ) -> String {
            let id = format!("{}:{}", from, name);
            self.relations.push(Relation {
                id: id.clone(),
                from: from.into(),
                to: to.into(),
                name: name.into(),
                kind,
                fields: vec![],
            });
            id
        }

        pub fn get_relation(&self, from: &str, name: &str) -> Option<&Relation> {
            let mut result = None;
            for x in self.relations.iter() {
                if x.from == from && x.name == name {
                    result = Some(x);
                    break;
                };
            }
            result
        }

        pub fn map_field(&mut self, id: &str, from_field: &str, to_field: &str) {
            for x in self.relations.iter_mut() {
                if x.id == id {
                    x.fields.push((from_field.into(), to_field.into()));
                    break;
                };
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct DataModel {
    name: String,
    tables: Vec<Table>,
}

impl DataModel {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            tables: vec![],
        }
    }

    pub fn table(mut self, tab: Table) -> Self {
        self.tables.push(tab);
        self
    }

    pub fn tables(&self) -> std::slice::Iter<'_, Table> {
        self.tables.iter()
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    // #[instrument(skip(self))]
    pub fn build(&mut self) {
        let dep_list = self.extract_dependent_fields();

        for (dep_tab, dep_fld, lu_tab, lu_fld) in dep_list.iter() {
            debug!("dependent list {}.{}", dep_tab, dep_fld);
            if let Some(tab) = self.find_table_mut(dep_tab.as_str()) {
                if let Some(Field { fieldtype, .. }) = tab.find_field_mut(dep_fld.as_str()) {
                    match fieldtype {
                        FieldType::DependentList(l) => {
                            debug!("add list field {} to be defined.", dep_fld);
                            l.push(DependentListItem {
                                table: lu_tab.clone(),
                                on_field: lu_fld.clone(),
                            });
                        }
                        _ => {
                            debug!("fieldtype ignored {:?}", fieldtype);
                        }
                    }
                } else {
                    debug!("list field {} to be defined.", dep_fld);
                    tab.add_field(
                        dep_fld,
                        false,
                        FieldType::DependentList(vec![DependentListItem {
                            table: lu_tab.clone(),
                            on_field: lu_fld.clone(),
                        }]),
                    );
                }
            } else {
                error!("table {} not found", dep_tab);
            }
        }
    }

    pub fn find_table(&self, tab_name: &str) -> Option<&Table> {
        self.tables().find(|x| x.name() == tab_name)
    }

    pub fn find_table_mut(&mut self, tab_name: &str) -> Option<&mut Table> {
        self.tables.iter_mut().find(|x| x.name() == tab_name)
    }

    fn extract_dependent_fields(&self) -> Vec<(String, String, String, String)> {
        let mut dep_list = vec![];
        // gather missing fields form refereneces
        for t in self.tables.iter() {
            for f in t.fields.iter() {
                match &f.fieldtype {
                    FieldType::Lookup { table, as_field } => {
                        dep_list.push((
                            table.clone(),
                            as_field.clone(),
                            t.name.clone(),
                            f.name.clone(),
                        ));
                    }
                    //FieldType::DependentList { table, on_field } =>
                    // FieldType::ReferenceList { table, via_table } => todo!(),
                    _ => {}
                }
            }
        }
        dep_list
    }

    pub fn dump(&self) {
        trace!("Model Dump: {}", self.name);
        for t in self.tables() {
            trace!("Table {}", t.name());
            for f in t.fields() {
                trace!(
                    "   {:2} {:20} {:?}",
                    if f.key { "o" } else { " " },
                    f.name(),
                    f.fieldtype
                );
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Table {
    name: String,
    fields: Vec<Field>,
}

impl Table {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            fields: vec![],
        }
    }

    pub fn field(mut self, arg: &str, key: bool, fieldtype: FieldType) -> Self {
        self.fields.push(Field {
            name: arg.into(),
            key,
            fieldtype,
        });
        self
    }

    pub fn add_field(&mut self, arg: &str, key: bool, fieldtype: FieldType) {
        self.fields.push(Field {
            name: arg.into(),
            key,
            fieldtype,
        });
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn fields(&self) -> std::slice::Iter<'_, Field> {
        self.fields.iter()
    }

    pub fn key(&self) -> impl Iterator<Item = &str> {
        self.fields
            .iter()
            .filter(|x| x.key)
            .map(|x| x.name.as_str())
    }

    pub fn find_field(&self, as_str: &str) -> Option<&Field> {
        self.fields().find(|x| x.name() == as_str)
    }
    pub fn find_field_mut(&mut self, as_str: &str) -> Option<&mut Field> {
        self.fields.iter_mut().find(|x| x.name() == as_str)
    }
}

#[derive(Debug, Clone)]
pub struct DependentListItem {
    pub table: String,
    pub on_field: String,
}

#[derive(Debug, Clone)]
pub enum FieldType {
    Text(usize),
    Number,
    Lookup { table: String, as_field: String },
    DependentList(Vec<DependentListItem>),
    ReferenceList { table: String, via_table: String },
}
#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub key: bool,
    pub fieldtype: FieldType,
}

impl Field {
    pub fn new(name: &str, key: bool, fieldtype: FieldType) -> Self {
        Self {
            name: name.into(),
            fieldtype,
            key,
        }
    }
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}
