use std::collections::HashMap;

use pjl_odata::ODataQuery;
use pjl_pg::Database;

#[tokio::main]
async fn main() -> Result<(), String> {
    if let Ok(mut db) =
        Database::new("host=localhost user=postgres password=Kennwort01 dbname=rk").await
    {
        let mut q = ODataQuery::new_from(
            "actor",
            &HashMap::from([("$filter".into(), "Name eq 'AJ Applegate'".into())]),
        );
        q.add_condition("name", "eq", &"Danny D".into());
        let result = db.select(q).await?;
        let mut out = String::new();
        result.dump(&mut out);
        println!("{out}");
    }
    Ok(())
}
