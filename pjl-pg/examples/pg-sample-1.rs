
use std::collections::HashMap;

use pjl_odata::ODataQuery;
use pjl_pg::Database;
use pjl_tab::Table;

#[tokio::main]
async fn main() {
    let mut db = Database::new("host=localhost user=postgres password=Kennwort01 dbname=rk").await;
    let mut q = ODataQuery::new_from(
        "actor",
        &HashMap::from([("$filter".into(), "Name eq 'AJ Applegate'".into())]),
    );
    assert!(db.connected());
    q.add_condition("name", "eq", "Danny D");
    let result = db.select(q).await;
    let mut out = String::new();
    result.dump(&mut out);
    println!("{out}");
}

