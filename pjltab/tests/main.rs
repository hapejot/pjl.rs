use memuse::DynamicUsage;
use pjltab::*;
#[test]
fn create() {
    let t = Table::new();
    assert_eq!(Ok(()), t.add_column("id"));
    assert!(t.add_column("ID").is_err());
    let row = t.new_row();
    let rid: usize = row.id();
    assert_eq!(rid, 1);
    row.set("ID", "Test");
    // row["ID"] = "Test";
    assert_eq!(row.get("ID"), Some(String::from("Test")));

    let cols = row.columns();
    assert_eq!(cols, vec!["id"]);

    row.set("NAME", "Müller");
    println!("{:#?}", t);
    let cols = row.columns();
    assert_eq!(cols, vec!["id", "name"]);
    assert_eq!(row.get("name"), Some(String::from("Müller")));

    let row = t.new_row();
    row.set("name", "Jaeckel");

    let start = std::time::Instant::now();
    let mut buf = String::new();
    t.dump(&mut buf);

    assert_eq!(
        "+----+-------+\n|id  |name   |\n+----+-------+\n|Test|Müller |\n|    |Jaeckel|\n+----+-------+\n",
        buf
    );
    eprintln!(
        "Time elapsed: {} ms",
        start.elapsed().as_secs_f64() * 1000f64
    );
    eprintln!("{buf}");
    eprintln!("size of output buffer: {}", buf.dynamic_usage());
    eprintln!("size of table {}", t.size());

    eprintln!("usage of table: {}", t.dynamic_usage());
}
