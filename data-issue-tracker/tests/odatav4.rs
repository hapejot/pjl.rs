use data_issue_tracker::odatav4::parse_odata_entity_path;

#[test]
fn test_valid_entity_path() {
    let res = parse_odata_entity_path("Issue('123')");
    assert_eq!(res, vec![("Issue".to_string(), Some("'123'".to_string()))]);

    let res = parse_odata_entity_path("User(42)");
    assert_eq!(res, vec![("User".to_string(), Some("42".to_string()))]);

    let res = parse_odata_entity_path("Entity(key-value)");
    assert_eq!(
        res,
        vec![("Entity".to_string(), Some("key-value".to_string()))]
    );

    let res = parse_odata_entity_path("A(1)/B(2)/C(3)");
    assert_eq!(
        res,
        vec![
            ("A".to_string(), Some("1".to_string())),
            ("B".to_string(), Some("2".to_string())),
            ("C".to_string(), Some("3".to_string())),
        ]
    );

    assert_eq!(
        parse_odata_entity_path("A(1)/B(2)/Broken"),
        vec![
            ("A".to_string(), Some("1".to_string())),
            ("B".to_string(), Some("2".to_string())),
            ("Broken".to_string(), None),
        ]
    );
    assert_eq!(
        parse_odata_entity_path("NoParenthesis"),
        vec![("NoParenthesis".to_string(), None)]
    );
}

#[test]
fn test_invalid_entity_path() {
    assert_eq!(parse_odata_entity_path("Entity()"), vec![]);
    assert_eq!(parse_odata_entity_path("Entity(123"), vec![]);
    assert_eq!(parse_odata_entity_path("Entity123)"), vec![]);
    assert_eq!(parse_odata_entity_path("Entity(123))"), vec![]);
}
