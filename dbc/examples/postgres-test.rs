use dbc::model::*;
use dbc::postgres::*;
use postgres::{Client, NoTls};

fn main() -> Result<(), String> {
    let mut client = sql_check(
        Client::configure()
            .host("localhost")
            .user("postgres")
            .password("Kennwort--01")
            .connect(NoTls),
    )?;

    let schema = read_schema("sample.schema");
    let schema2 = collect_tables(&mut client)?;
    println!("{schema2:#?}");
    let diff = diff_schema(&schema2, &schema);
    for d in diff {
        match d {
            SchemaDiff::NewTable(n) => {
                let tt = schema.get_table(&n).unwrap();
                let sql = build_create_table(&tt).map_err(|e| e.to_string())?;

                sql_check(client.execute(&sql, &[]))?;
            }
            SchemaDiff::NewField(tname, fname) => {
                let tt = schema.get_table(&tname).unwrap();
                let f = tt.get_field(&fname).unwrap();
                let sql = build_add_field(&tname, f)?;
                sql_check(client.execute(&sql, &[]))?;
            }
        }
    }

    // sql_check(client.batch_execute(
    //     "CREATE TABLE person (
    //                 id      SERIAL PRIMARY KEY,
    //                 name    TEXT NOT NULL,
    //                 data    BYTEA )",
    // ))?;

    let mut row = edm::structure::StructureValue::new_with_type("person");
    // row["id"] = 2.into();
    row["name"] = "Ferris".into();
    update(&mut client, &schema, &row).or_else(|_| insert(&mut client, &schema, &row))?;
    // sql_check(client.execute(
    //     "INSERT INTO person (name, data) VALUES ($1, $2)",
    //     &[&name, &data],
    // ))?;

    for row in sql_check(client.query("SELECT id, name, data FROM person", &[]))? {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
        let data: Option<&[u8]> = row.get(2);

        println!("found person: {} {} {:?}", id, name, data);
    }
    Ok(())
}
