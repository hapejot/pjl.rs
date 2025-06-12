use std::collections::HashMap;

use pjl_odata::ODataQuery;
use pjl_pg::Database;
use pjl_tab::Table;

#[tokio::test]
async fn connect() {
    if let Ok(db) = Database::new("host=localhost user=postgres password=Kennwort01").await {
        assert!(db.connected());
    }
}

#[tokio::test]
async fn select() {
    if let Ok(mut db) =
        Database::new("host=localhost user=postgres password=Kennwort01 dbname=rk").await
    {
        let mut q = ODataQuery::new_from(
            "actor",
            &HashMap::from([("$filter".into(), "Name eq 'AJ Applegate'".into())]),
        );
        q.add_condition("name", "eq", &"Danny D".into());
        let result = db.select(q).await;
        let mut out = String::new();
        result.dump(&mut out);
        println!("{out}");
    }
}

#[tokio::test]
async fn modify() {
    let yaml = r#"
- id: '42'
  name: Peter Jaeckel
  birthplace: Hannover
  gender: M
- id: '17'
  birthplace: Burgwedel-Großburgwedel
  gender: F
  name: Karin J."#;

    let tab2: Table = serde_yaml::from_str(&yaml).unwrap();

    if let Ok(mut db) =
        Database::new("host=localhost user=postgres password=Kennwort01 dbname=rk").await
    {
        // let tab = Table::new();
        // let row = tab.new_row();
        // row.set("id", "42");
        // row.set("name", "Peter J.");
        // let row = tab.new_row();
        // row.set("id", "17");
        // row.set("name", "Karin J.");
        println!("{}", serde_yaml::to_string(&tab2).unwrap());
        db.modify("actor", tab2).await;
    }
}

/*
SELECT a.attname as field
FROM   pg_index i
JOIN   pg_attribute a ON a.attrelid = i.indrelid
                     AND a.attnum   = ANY(i.indkey)
WHERE  i.indrelid = 'actor'::regclass
AND    i.indisprimary;
*/

#[tokio::test]
async fn ind() {
    if let Ok(mut db) =
        Database::new("host=localhost user=postgres password=Kennwort01 dbname=rk").await
    {
        let result = db.read_primary_key("actor_scene").await;

        eprintln!("{result:?}");
    }
}
