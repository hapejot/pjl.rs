use dbc::model::*;
use dbc::sqlite::*;
use rusqlite::Connection;

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() -> Result<(), String> {
    let conn = Connection::open("demo.sqlite").map_err(|x| x.to_string())?;

    let schema = sample_schema();
    write_schema(&schema, "sample.schema");
    let schema2 = collect_tables(&conn)?;
    let diff = diff_schema(&schema2, &schema);
    for d in diff {
        match d {
            SchemaDiff::NewTable(n) => {
                let tt = schema.get_table(&n).unwrap();
                let sql = build_create_table(&tt).map_err(|e| e.to_string())?;

                sql_check(conn.execute(&sql, ()))?;
            }
            SchemaDiff::NewField(tname, fname) => {
                let tt = schema.get_table(&tname).unwrap();
                let f = tt.get_field(&fname).unwrap();
                let sql = build_add_field(&tname, f)?;
                sql_check(conn.execute(&sql, ()))?;
            }
        }
    }

    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };
    sql_check(conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        (&me.name, &me.data),
    ))?;

    let mut stmt = conn
        .prepare("SELECT id, name, data FROM person")
        .map_err(|x| x.to_string())?;
    let person_iter = stmt
        .query_map([], |row| {
            Ok(Person {
                id: row.get(0)?,
                name: row.get(1)?,
                data: row.get(2)?,
            })
        })
        .map_err(|x| x.to_string())?;

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
    Ok(())
}

fn sample_schema() -> Schema {
    let mut schema = Schema::new();
    let mut table = Table::new("person");
    table.add(Field::new("id", "INTEGER").key());
    table.add(Field::new("name", "TEXT"));
    table.add(Field::new("data", "BLOB").nullable());
    table.add(Field::new("birth_date", "date").nullable());
    schema.add(table.clone());
    schema
}

