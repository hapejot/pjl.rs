use std::default;

use pjl_tab::Table;
use serde::{de::MapAccess, Deserialize, Deserializer};




#[derive(Debug, Deserialize)]
struct User {
    id: i64,
    username: String,
    access_token: String,
}

#[test]
fn deser() {
    let t = Table::new();
    let r = t.new_row();
    r.set("Spalte1", "A");
    r.set("Spalte2", "B");

    let s = serde_json::to_string(&t).unwrap();

    assert_eq!("[{\"spalte1\":\"A\",\"spalte2\":\"B\"}]", s);
}

#[test]
fn deserialize_test() {
    let tab = Table::new();
    let r = tab.new_row();
    r.set("id", "17");
    r.set("username", "Peter");
    r.set("access_token", "Demo");

    let v: Vec<User> = pjl_tab::de::extract_from_table(&tab).unwrap();
    assert_eq!(1, v.len());
    let s = format!("{v:?}");
    assert_eq!(
        "[User { id: 17, username: \"Peter\", access_token: \"Demo\" }]",
        s
    );
}


#[test]
fn deserialize_test_multiple() {
    let tab = Table::new();
    let r = tab.new_row();
    r.set("id", "17");
    r.set("username", "Peter");
    r.set("access_token", "Demo");
    let r = tab.new_row();
    r.set("id", "42");
    r.set("username", "Karin");
    r.set("access_token", "What?");

    let v: Vec<User> = pjl_tab::de::extract_from_table(&tab).unwrap();
    assert_eq!(2, v.len());
    let s = format!("{v:?}");
    assert_eq!(
        "[User { id: 17, username: \"Peter\", access_token: \"Demo\" }, User { id: 42, username: \"Karin\", access_token: \"What?\" }]",
        s
    );
}
