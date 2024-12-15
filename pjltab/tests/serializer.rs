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

    let t = pjltab::ser::table_from(&p).unwrap();
    assert_eq!(2, t.lines());
    let r = t.row(1);
    assert_eq!("Jaeckel", r.get("name2").unwrap());
    assert_eq!("Peter", r.get("name1").unwrap());
    let r = t.row(2);
    assert_eq!("Karin", r.get("name1").unwrap());
    assert_eq!("Lueg", r.get("name2").unwrap());
}