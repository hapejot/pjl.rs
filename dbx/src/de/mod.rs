use derow::Deserializer;
use serde::de::DeserializeOwned;

mod derow;

pub fn from_row<T>(row: &rusqlite::Row) -> crate::error::Result<T>
where
    T: DeserializeOwned,
{
    let columns = row
        .as_ref()
        .column_names()
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();

    let de = Deserializer::from_row(row, &columns);
    let r = T::deserialize(de);
    r
}
