use std::fmt;

use pjl_tab::map::ValueMapping;
use regex::Regex;
use serde::{de::{self, Visitor}, Deserialize, Serialize, Serializer};

/// Represents a standard SQL type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SqlType {
    Integer,
    BigInt,
    SmallInt,
    Numeric,
    Decimal,
    Real,
    DoublePrecision,
    Varchar(Option<usize>),
    Char(Option<usize>),
    Text,
    Boolean,
    Date,
    Timestamp,
    TimestampWithTimeZone,
    Time,
    TimeWithTimeZone,
    Json,
    Jsonb,
    Uuid,
    Bytea,
}

impl SqlType {
    /// Parses a SQL type string into a `SqlType`.
    pub fn parse(type_str: &str) -> Self {
        let type_str = type_str.trim();
        let re = Regex::new(r"^([a-zA-Z\s]+)\s*(?:\(\s*(\d+)\s*\))?$").unwrap();
        if let Some(caps) = re.captures(type_str) {
            let name_str = caps.get(1).unwrap().as_str();
            let size = caps.get(2).map(|m| m.as_str().parse::<usize>().unwrap());
            match name_str.to_uppercase().as_str() {
                "INTEGER" => SqlType::Integer,
                "BIGINT" => SqlType::BigInt,
                "SMALLINT" => SqlType::SmallInt,
                "NUMERIC" => SqlType::Numeric,
                "DECIMAL" => SqlType::Decimal,
                "REAL" => SqlType::Real,
                "DOUBLE PRECISION" => SqlType::DoublePrecision,
                "VARCHAR" => SqlType::Varchar(size),
                "CHAR" => SqlType::Char(size),
                "TEXT" => SqlType::Text,
                "BOOLEAN" => SqlType::Boolean,
                "DATE" => SqlType::Date,
                "TIMESTAMP" => SqlType::Timestamp,
                "TIMESTAMP WITH TIME ZONE" => SqlType::TimestampWithTimeZone,
                "TIME" => SqlType::Time,
                "TIME WITH TIME ZONE" => SqlType::TimeWithTimeZone,
                "JSON" => SqlType::Json,
                "JSONB" => SqlType::Jsonb,
                "UUID" => SqlType::Uuid,
                "BYTEA" => SqlType::Bytea,
                _ => panic!("Invalid SQL type name: '{}' from '{}'", name_str, type_str),
            }
        } else {
            match type_str.to_uppercase().as_str() {
                "INTEGER" => SqlType::Integer,
                "BIGINT" => SqlType::BigInt,
                "SMALLINT" => SqlType::SmallInt,
                "NUMERIC" => SqlType::Numeric,
                "DECIMAL" => SqlType::Decimal,
                "REAL" => SqlType::Real,
                "DOUBLE PRECISION" => SqlType::DoublePrecision,
                "TEXT" => SqlType::Text,
                "BOOLEAN" => SqlType::Boolean,
                "DATE" => SqlType::Date,
                "TIMESTAMP" => SqlType::Timestamp,
                "TIMESTAMP WITH TIME ZONE" => SqlType::TimestampWithTimeZone,
                "TIME" => SqlType::Time,
                "TIME WITH TIME ZONE" => SqlType::TimeWithTimeZone,
                "JSON" => SqlType::Json,
                "JSONB" => SqlType::Jsonb,
                "UUID" => SqlType::Uuid,
                "BYTEA" => SqlType::Bytea,
                _ => panic!("Invalid SQL type name: '{}'", type_str),
            }
        }
    }

    /// Returns a string representation of the `SqlType`.
    pub fn to_string(&self) -> String {
        match self {
            SqlType::Integer => "INTEGER".to_string(),
            SqlType::BigInt => "BIGINT".to_string(),
            SqlType::SmallInt => "SMALLINT".to_string(),
            SqlType::Numeric => "NUMERIC".to_string(),
            SqlType::Decimal => "DECIMAL".to_string(),
            SqlType::Real => "REAL".to_string(),
            SqlType::DoublePrecision => "DOUBLE PRECISION".to_string(),
            SqlType::Varchar(Some(size)) => format!("VARCHAR({})", size),
            SqlType::Varchar(None) => "VARCHAR".to_string(),
            SqlType::Char(Some(size)) => format!("CHAR({})", size),
            SqlType::Char(None) => "CHAR".to_string(),
            SqlType::Text => "TEXT".to_string(),
            SqlType::Boolean => "BOOLEAN".to_string(),
            SqlType::Date => "DATE".to_string(),
            SqlType::Timestamp => "TIMESTAMP".to_string(),
            SqlType::TimestampWithTimeZone => "TIMESTAMP WITH TIME ZONE".to_string(),
            SqlType::Time => "TIME".to_string(),
            SqlType::TimeWithTimeZone => "TIME WITH TIME ZONE".to_string(),
            SqlType::Json => "JSON".to_string(),
            SqlType::Jsonb => "JSONB".to_string(),
            SqlType::Uuid => "UUID".to_string(),
            SqlType::Bytea => "BYTEA".to_string(),
        }
    }
}

/// Creates a mapping from PostgreSQL data types to standard SQL data types.
///
/// This function defines a `ValueMapping` that translates common PostgreSQL data type names
/// (like "integer", "character varying(10)", etc.) to their corresponding standard SQL equivalents
/// (like "INTEGER", "VARCHAR(10)", etc.). This can be useful when you need to represent
/// PostgreSQL types in a more generic SQL context.
///
/// # Returns
///
/// A `ValueMapping<String, SqlType>` representing the mapping from PostgreSQL
/// to standard SQL types.
pub fn postgres_to_standard_sql_type_mapping() -> ValueMapping<String, SqlType> {
    ValueMapping::new_from(&[
        (String::from("integer"), SqlType::Integer),
        (String::from("int"), SqlType::Integer),
        (String::from("serial"), SqlType::Integer),
        (String::from("bigint"), SqlType::BigInt),
        (String::from("bigserial"), SqlType::BigInt),
        (String::from("smallint"), SqlType::SmallInt),
        (String::from("numeric"), SqlType::Numeric),
        (String::from("decimal"), SqlType::Decimal),
        (String::from("real"), SqlType::Real),
        (String::from("double precision"), SqlType::DoublePrecision),
        (String::from("character varying"), SqlType::Varchar(None)),
        (String::from("varchar"), SqlType::Varchar(None)),
        (String::from("character"), SqlType::Char(None)),
        (String::from("char"), SqlType::Char(None)),
        (String::from("text"), SqlType::Text),
        (String::from("boolean"), SqlType::Boolean),
        (String::from("bool"), SqlType::Boolean),
        (String::from("date"), SqlType::Date),
        (String::from("timestamp without time zone"), SqlType::Timestamp),
        (String::from("timestamp"), SqlType::Timestamp),
        (String::from("timestamp with time zone"), SqlType::TimestampWithTimeZone),
        (String::from("timestamptz"), SqlType::TimestampWithTimeZone),
        (String::from("time without time zone"), SqlType::Time),
        (String::from("time with time zone"), SqlType::TimeWithTimeZone),
        (String::from("timetz"), SqlType::TimeWithTimeZone),
        (String::from("json"), SqlType::Json),
        (String::from("jsonb"), SqlType::Jsonb),
        (String::from("uuid"), SqlType::Uuid),
        (String::from("bytea"), SqlType::Bytea),
    ])
}


impl Serialize for SqlType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for SqlType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct SqlTypeVisitor;

        impl<'de> Visitor<'de> for SqlTypeVisitor {
            type Value = SqlType;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a SQL type string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(SqlType::parse(value))
            }
        }

        deserializer.deserialize_str(SqlTypeVisitor)
    }
}