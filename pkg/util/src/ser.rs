use serde::Serializer;

pub fn serialize_option_bool_as_int<S>(
    value: &Option<bool>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(true) => serializer.serialize_i32(1),
        Some(false) | None => serializer.serialize_i32(0),
    }
}

pub fn ser_opt_as_str<S>(value: &Option<i32>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(v) => serializer.serialize_str(v.to_string().as_str()),
        None => serializer.serialize_str("null"),
    }
}

pub fn ser_as_str<S>(value: i32, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&value.to_string())
}

pub fn serialize_vec_as_comma_separated<S>(vec: &[String], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let joined = vec.join(",");
    serializer.serialize_str(&joined)
}
