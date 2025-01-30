pub fn serialize_option_bool_as_int<S>(
    value: &Option<bool>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match value {
        Some(true) => serializer.serialize_i32(1),
        Some(false) | None => serializer.serialize_i32(0),
    }
}
