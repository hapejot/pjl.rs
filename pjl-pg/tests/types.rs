use pjl_pg::types::{SqlType, postgres_to_standard_sql_type_mapping};

#[test]
fn test_sql_type_parse() {
    assert_eq!(SqlType::parse("integer"), SqlType::Integer);
    assert_eq!(SqlType::parse("VARCHAR(255)"), SqlType::Varchar(Some(255)));
    assert_eq!(SqlType::parse("char(10)"), SqlType::Char(Some(10)));
    assert_eq!(SqlType::parse("TEXT"), SqlType::Text);
    assert_eq!(SqlType::parse("varchar(20)"), SqlType::Varchar(Some(20)));
}

#[test]
fn test_sql_type_to_string() {
    assert_eq!(SqlType::Varchar(Some(255)).to_string(), "VARCHAR(255)");
    assert_eq!(SqlType::Integer.to_string(), "INTEGER");
    assert_eq!(SqlType::Char(None).to_string(), "CHAR");
}

#[test]
fn test_postgres_to_standard_sql_type_mapping() {
    let mapping = postgres_to_standard_sql_type_mapping();

    // Test some common mappings
    assert_eq!(mapping.map(&"integer".to_string()), Some(SqlType::Integer));
    assert_eq!(mapping.map(&"varchar".to_string()), Some(SqlType::Varchar(None)));
    assert_eq!(mapping.map(&"character varying".to_string()), Some(SqlType::Varchar(None)));
    assert_eq!(mapping.map(&"text".to_string()), Some(SqlType::Text));
    assert_eq!(mapping.map(&"boolean".to_string()), Some(SqlType::Boolean));
    assert_eq!(mapping.map(&"timestamp without time zone".to_string()), Some(SqlType::Timestamp));
    assert_eq!(mapping.map(&"timestamptz".to_string()), Some(SqlType::TimestampWithTimeZone));
    assert_eq!(mapping.map(&"jsonb".to_string()), Some(SqlType::Jsonb));

    // Test some alternative type names
    assert_eq!(mapping.map(&"int".to_string()), Some(SqlType::Integer));
    assert_eq!(mapping.map(&"bool".to_string()), Some(SqlType::Boolean));

    // Test a non-existent mapping
    assert_eq!(mapping.map(&"nonexistenttype".to_string()), None);
}

#[test]
fn test_reverse_mapping() {
    let mapping = postgres_to_standard_sql_type_mapping();

    assert_eq!(mapping.rmap(&SqlType::Integer), Some("integer".to_string()));
    assert_eq!(mapping.rmap(&SqlType::Varchar(None)), Some("character varying".to_string()));
    assert_eq!(mapping.rmap(&SqlType::Text), Some("text".to_string()));
    assert_eq!(mapping.rmap(&SqlType::Timestamp), Some("timestamp without time zone".to_string()));
    assert_eq!(mapping.rmap(&SqlType::TimestampWithTimeZone), Some("timestamp with time zone".to_string()));
    assert_eq!(mapping.rmap(&SqlType::Jsonb), Some("jsonb".to_string()));
    assert_eq!(mapping.rmap(&SqlType::Varchar(Some(10))), None);
}



#[test]
fn test_sql_type_serialize() {
    let sql_type = SqlType::Integer;
    let serialized = serde_yaml::to_string(&sql_type).unwrap();
    assert_eq!(serialized, "INTEGER\n");

    let sql_type = SqlType::Varchar(Some(255));
    let serialized = serde_yaml::to_string(&sql_type).unwrap();
    assert_eq!(serialized, "VARCHAR(255)\n");

    let sql_type = SqlType::Char(None);
    let serialized = serde_yaml::to_string(&sql_type).unwrap();
    assert_eq!(serialized, "CHAR\n");
}

#[test]
fn test_sql_type_deserialize() {
    let yaml = "\"INTEGER\"";
    let sql_type: SqlType = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(sql_type, SqlType::Integer);

    let yaml = "\"VARCHAR(255)\"";
    let sql_type: SqlType = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(sql_type, SqlType::Varchar(Some(255)));

    let yaml = "\"CHAR\"";
    let sql_type: SqlType = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(sql_type, SqlType::Char(None));

    let yaml = "\"TEXT\"";
    let sql_type: SqlType = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(sql_type, SqlType::Text);

    let yaml = "\"TIMESTAMP WITH TIME ZONE\"";
    let sql_type: SqlType = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(sql_type, SqlType::TimestampWithTimeZone);
}