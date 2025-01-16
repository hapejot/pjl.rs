use pjl_tab::Table;
use serde::Serialize;

#[derive(Serialize, Debug)]
struct Person {
    name1: String,
    name2: String,
}

#[test]
fn ser1() {
    let p = vec![
        Person {
            name1: "Peter".into(),
            name2: "Jaeckel".into(),
        },
        Person {
            name1: "Karin".into(),
            name2: "Lueg".into(),
        },
    ];

    let t = pjl_tab::ser::table_from(&p).unwrap();
    assert_eq!(2, t.lines());
    let r = t.row(1);
    assert_eq!("Jaeckel", r.get("name2").unwrap());
    assert_eq!("Peter", r.get("name1").unwrap());
    let r = t.row(2);
    assert_eq!("Karin", r.get("name1").unwrap());
    assert_eq!("Lueg", r.get("name2").unwrap());
    let mut out = String::new();
    t.dump(&mut out);
    eprintln!("{}", out);
}

#[test]
fn deser() {
    let t = Table::new();
    let r = t.new_row();
    r.set("Spalte1", "A");
    r.set("Spalte2", "B");
    let s = serde_json::to_string(&t).unwrap();

    assert_eq!("[{\"spalte1\":\"A\",\"spalte2\":\"B\"}]", s);
}
