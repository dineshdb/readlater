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

pub fn serialize_vec_as_comma_separated<S>(vec: &[String], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let joined = vec.join(",");
    serializer.serialize_str(&joined)
}
