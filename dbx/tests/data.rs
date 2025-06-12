use dbx::data::model::FieldType::Text;
use dbx::data::model::FieldType::Lookup;
use dbx::data::{
    model::{DataModel, Table},
    Query, WhereCondition, WhereExpr,
};



#[test]
fn query() {
    let w = WhereCondition::new().and(WhereExpr::Equals("type".into(), "Null".into()));

    let q = Query::new("object", vec!["id", "type"], w);
    let sql = q.get_sql();
    assert_eq!(sql, "SELECT id,type FROM object WHERE type = ?");
    let _p = q.get_params();
    // assert_eq!(p, vec!["Null"]);
}

#[test]
fn data_model() {
    let mut model = DataModel::new("Person");
    let tab = Table::new("person")
        .field("id", true, Text(20))
        .field("name1", false, Text(100))
        .field("name2", false, Text(100))
        .field("name3", false, Text(100))
        .field("name4", false, Text(100));
    model = model
        .table(tab)
        .table(
            Table::new("email")
                .field("id", true, Text(20))
                .field("person", false, Lookup { table: "person".into(), as_field: "communication".into() })
                .field("role", false, Text(100))
                .field("address", false, Text(100)),
        )
        .table(
            Table::new("phone")
                .field("id", true, Text(20))
                .field("person", false, Lookup { table: "person".into(), as_field: "communication".into() })
                .field("role", false, Text(100))
                .field("number", false, Text(100)),
        );


        assert_eq!("person", model.tables().next().unwrap().name());
}
