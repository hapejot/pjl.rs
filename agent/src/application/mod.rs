use lazy_static::*;
use rusqlite::Connection;
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex, MutexGuard, PoisonError},
};
use tracing::{trace};

lazy_static! {
    pub static ref APP: Application = Application::new();
}

#[derive(Debug, Clone)]
pub enum Value {
    Str(String),
    Obj(usize),
}

struct Data {
    pub entries: BTreeMap<usize, BTreeMap<String, Value>>,
    pub next_objid: usize,
}

impl Data {
    pub fn new() -> Self {
        let r = Self {
            entries: BTreeMap::new(),
            next_objid: 2,
        };
        r
    }

    fn new_obj(&mut self) -> usize {
        let objid = self.next_objid;
        self.entries.insert(self.next_objid, BTreeMap::new());
        self.next_objid += 1;
        objid
    }

    fn dump(&self) {
        for (id, obj) in self.entries.iter() {
            trace!("-> {:}", id);
            for (k, v) in obj.iter() {
                trace!("--- {:} {:?}", k, v);
            }
        }
    }

    fn get(&self, objid: usize, key: &String) -> Option<Value> {
        if let Some(x) = self.entries.get(&objid) {
            if let Some(y) = x.get(key) {
                Some(y.clone())
            } else {
                None
            }
        } else {
            None
        }
    }

    fn set(&mut self, objid: usize, key: String, value: Value) {
        if let Some(obj) = self.entries.get_mut(&objid) {
            obj.insert(key, value);
        }
    }

    fn save_obj_to_db(&self, objid: usize) {
        let con = Connection::open("agent.sqlite").expect("open");
        let mut stmt = con
            .prepare("delete from dir where objid = ?")
            .expect("prepare");
        stmt.execute([objid]).expect("execute");
        if let Some(obj) = self.entries.get(&objid) {
            for (key, value) in obj.iter() {
                match value {
                    Value::Str(s) => {
                        let mut insert_stmt = con
                            .prepare(
                                "insert into dir(objid, name, value, datatype) values (?,?,?,?)",
                            )
                            .expect("prepare");
                        insert_stmt
                            .execute((objid, key, s, "string"))
                            .expect("execute");
                    }
                    Value::Obj(o) => {
                        let mut insert_stmt = con
                            .prepare(
                                "insert into dir(objid, name, value, datatype) values (?,?,?,?)",
                            )
                            .expect("prepare");
                        insert_stmt
                            .execute((objid, key, o, "obj"))
                            .expect("execute");
                    }
                }
            }
        }
    }

    fn load_obj_from_db(&mut self, objid: usize) {
        let con = Connection::open("agent.sqlite").expect("open");
        let mut stmt = con
            .prepare("select name, value, datatype from dir where objid = ?")
            .expect("prepare");
        let mut rows = stmt.query([objid]).expect("query");

        let mut obj = BTreeMap::<String, Value>::new();

        while let Some(row) = rows.next().expect("next") {
            let name = row.get::<usize, String>(0).expect("key").clone();
            let datatype = row.get::<usize, String>(2).expect("datatype");
            match datatype.as_str() {
                "string" => {
                    let value = row.get::<usize, String>(1).expect("value").clone();
                    obj.insert(name, Value::Str(value));
                }
                "obj" => {
                    let value = row.get::<usize, usize>(1).expect("value").clone();
                    obj.insert(name, Value::Obj(value));
                }
                _ => {}
            }
        }
        self.entries.insert(1, obj);
    }

    fn get_all(&self, objid: usize) -> BTreeMap<String, Value> {
        if let Some(map) = self.entries.get(&objid) {
            map.clone()
        } else {
            BTreeMap::new()
        }
    }
}

struct DataPort {
    pub d: Mutex<Data>,
}

impl DataPort {
    fn new() -> Self {
        Self {
            d: Mutex::new(Data::new()),
        }
    }
}

#[derive(Clone)]
pub struct Application {
    port: Arc<DataPort>,
}

type DataMutexGuard<'a> = MutexGuard<'a, Data>;

impl Application {
    pub fn new() -> Self {
        Self {
            port: Arc::new(DataPort::new()),
        }
    }

    pub fn init(&self) {
        if let Ok(mut data) = self.get_lock() {
            data.load_obj_from_db(1);
        };
    }

    pub fn dump(&self) {
        if let Ok(x) = self.get_lock() {
            x.dump();
        }
    }

    fn get_lock(&self) -> Result<DataMutexGuard<'_>, PoisonError<DataMutexGuard<'_>>> {
        self.port.d.lock()
    }

    pub fn set(&self, objid: usize, key: String, value: Value) {
        if let Ok(mut d) = self.get_lock() {
            d.set(objid, key, value);
        }
    }

    pub fn get(&self, objid: usize, key: &String) -> Option<Value> {
        if let Ok(d) = self.get_lock() {
            d.get(objid, key)
        } else {
            None
        }
    }

    pub fn save_obj(&self, objid: usize) {
        if let Ok(d) = self.get_lock() {
            d.save_obj_to_db(objid);
        }
    }

    pub fn new_obj(&self) -> usize {
        if let Ok(mut d) = self.get_lock() {
            d.new_obj()
        } else {
            0
        }
    }

    pub fn get_entries(&self, objid: usize) -> BTreeMap<String, Value> {
        if let Ok(d) = self.get_lock() {
            d.get_all(objid)
        } else {
            BTreeMap::new()
        }
    }

}
