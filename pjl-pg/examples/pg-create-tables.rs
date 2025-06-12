use edm::*;
use pjl_pg::Database;

#[tokio::main]
async fn main() {
    if let Ok(mut db) =
        Database::new("host=localhost user=postgres password=Kennwort01 dbname=rk").await
    {
        let mut s = Schema::new();
        s.new_entity("todo");
        // s.new_property("todo", "id");
        s.new_property("todo", "title");
        s.new_property("todo", "status");
        s.new_property("todo", "description");
        // s.new_key("todo", &["id"]);
        println!("{s:#?}");

        let _result = db.activate(s).await;
    }
}
