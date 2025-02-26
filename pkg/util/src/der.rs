use serde::{
    de::{self, Unexpected},
    Deserialize, Deserializer,
};
use std::str::FromStr;

pub fn bool_from_number<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let value: i64 = Deserialize::deserialize(deserializer)?;
    match value {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(de::Error::invalid_value(
            Unexpected::Signed(value),
            &"expected truthy or falsy string",
        )),
    }
}

pub fn from_string<'de, D, T: FromStr>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T::Err: std::fmt::Display,
{
    let value: &str = Deserialize::deserialize(deserializer)?;
    value.parse::<T>().map_err(de::Error::custom)
}

pub fn opt_from_string<'de, D, T: FromStr>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T::Err: std::fmt::Display,
{
    let v = from_string(deserializer);
    match v {
        Ok(v) => Ok(Some(v)),
        Err(_) => Ok(None),
    }
}
