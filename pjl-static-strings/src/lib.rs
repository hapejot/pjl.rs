use std::{collections::HashSet, sync::Mutex};

use once_cell::sync::Lazy;
use tracing::*;

pub struct StringTable {
    cache: Mutex<HashSet<&'static str>>,
}

impl StringTable {
    pub fn get(name: &str) -> &'static str {
        let mut lck = STRING_TABLE.cache.lock().unwrap();
        match lck.get(name) {
            Some(s) => *s,
            None => {
                let s0 = Box::new(name.to_string());
                let s1: &'static String = Box::leak(s0);
                lck.insert(s1);
                s1
            }
        }
    }

    pub fn stats() {
        trace!("Strings in memory:");
        for x in STRING_TABLE.cache.lock().unwrap().iter() {
            trace!("{}", x);
        }
    }
}

static STRING_TABLE: Lazy<StringTable> = Lazy::new(|| StringTable {
    cache: Mutex::new(HashSet::new()),
});

#[cfg(test)]
mod test {
    use super::StringTable;

    #[test]
    fn selector() {
        let sel1 = StringTable::get("a:b:");
        let sel2 = StringTable::get(format!("{}:{}:", "a", "b").as_str());
        assert_eq!(sel1, sel2);
        assert_eq!(sel1.as_ptr(), sel2.as_ptr());
    }
}
