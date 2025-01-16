use std::collections::HashMap;

use pjl_odata::ODataQuery;
use pjl_pg::Database;
use pjl_tab::Table;

#[tokio::test]
async fn connect() {
    let db = Database::new("host=localhost user=postgres password=Kennwort01").await;
    assert!(db.connected());
}

#[tokio::test]
async fn select() {
    let mut db = Database::new("host=localhost user=postgres password=Kennwort01 dbname=rk").await;
    let mut q = ODataQuery::new_from(
        "actor",
        &HashMap::from([("$filter".into(), "Name eq 'AJ Applegate'".into())]),
    );
    q.add_condition("name", "eq", "Danny D");
    let result = db.select(q).await;
    let mut out = String::new();
    result.dump(&mut out);
    println!("{out}");
}

#[tokio::test]
async fn modify() {
    let db = Database::new("host=localhost user=postgres password=Kennwort01 dbname=rk").await;
    let tab = Table::new();
    let row = tab.new_row();
    row.set("id", "42");
    row.set("name", "Peter");
    db.modify("actor", tab);
}
