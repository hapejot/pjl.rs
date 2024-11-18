use pjltab::*;

#[test]
fn create() {
    let t = Table::new();
    assert_eq!(Ok(()), t.add_column("id"));
    assert!(t.add_column("ID").is_err());
    let row = t.new_row();
    let rid:usize = row.id();
    assert_eq!(rid, 1);
    row.set("ID", "Test");
    // row["ID"] = "Test";
    assert_eq!(row.get("ID"), Some(String::from("Test")));

    row.set("NAME", "Müller");
    println!("{:#?}", t);
    assert_eq!(row.get("name"), Some(String::from("Müller")));
}