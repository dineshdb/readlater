use std::str::FromStr;

use serde::{
    de::{self, Unexpected},
    Deserialize, Deserializer,
};

pub fn bool_from_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let value: &str = Deserialize::deserialize(deserializer)?;
    match value {
        "0" => Ok(false),
        "1" => Ok(true),
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(de::Error::invalid_value(
            Unexpected::Str(value),
            &"expected truthy or falsy string",
        )),
    }
}

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

pub fn i32_from_string<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    from_string(deserializer)
}

pub fn u64_from_string<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    from_string(deserializer)
}
