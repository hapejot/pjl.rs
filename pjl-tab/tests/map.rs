use pjl_tab::map::ValueMapping;

#[test]
fn setup() {
    let m: ValueMapping<String, usize> = ValueMapping::new();
}

#[test]
fn setup_with_initial_data() {
    let m = create_mapping();

    assert_eq!(m.map(&"VARCHAR"), Some(1));
    assert_eq!(m.map(&"INTEGER"), Some(2));
    assert_eq!(m.map(&"TEXT"), None);
}

fn create_mapping() -> ValueMapping<&'static str, i32> {
    let m = ValueMapping::new_from(&[("VARCHAR", 1), ("INTEGER", 2)]);
    m
}


#[test]
fn reverse_mapping() {
    let m = ValueMapping::new_from(&[
        ("VARCHAR", "character varying"),
        ("TIMESTAMP", "timestamp without time zone"),
        ("INTEGER", "integer"),
    ]);

    assert_eq!(m.rmap(&"integer"), Some("INTEGER"));
    assert_eq!(m.rmap(&"timestamp without time zone"), Some("TIMESTAMP"));
    assert_eq!(m.rmap(&"foo"), None);
}
