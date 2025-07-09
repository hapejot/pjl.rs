use data_issue_tracker::odatav4::parse_odata_entity_path;

#[test]
fn serde_json_value() {
    let after_entity = "'123'";
    let r = serde_yaml::from_str::<serde_json::Value>(after_entity);
    match r {
        Ok(v) => assert_eq!(v, serde_json::Value::String("123".to_string())),
        Err(e) => assert_eq!(e.to_string(), ""),
    }
}

#[test]
fn test_valid_entity_path() {
    let res = parse_odata_entity_path("Issue('123')");
    assert_eq!(
        res,
        vec![(
            "Issue".to_string(),
            Some(serde_json::Value::String("123".to_string()))
        )]
    );

    let res = parse_odata_entity_path("User(42)");
    assert_eq!(
        res,
        vec![(
            "User".to_string(),
            Some(serde_json::Value::Number(42.into()))
        )]
    );

    let res = parse_odata_entity_path("Entity(key-value)");
    assert_eq!(
        res,
        vec![(
            "Entity".to_string(),
            Some(serde_json::Value::String("key-value".to_string()))
        )]
    );

    let res = parse_odata_entity_path("A(1)/B(2)/C(3)");
    assert_eq!(
        res,
        vec![
            ("A".to_string(), Some(serde_json::Value::Number(1.into()))),
            ("B".to_string(), Some(serde_json::Value::Number(2.into()))),
            ("C".to_string(), Some(serde_json::Value::Number(3.into()))),
        ]
    );

    assert_eq!(
        parse_odata_entity_path("A(1)/B(2)/Broken"),
        vec![
            ("A".to_string(), Some(serde_json::Value::Number(1.into()))),
            ("B".to_string(), Some(serde_json::Value::Number(2.into()))),
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
