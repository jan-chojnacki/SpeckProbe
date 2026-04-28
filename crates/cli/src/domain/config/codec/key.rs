use serde::Deserialize;

pub fn serialize_as_hex<S>(data: &[u8], s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let hex = data
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .join(" ");
    s.serialize_str(&hex)
}

pub fn deserialize_from_hex<'de, D>(d: D) -> Result<Vec<u8>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(d)?;
    s.split_whitespace()
        .map(|b| u8::from_str_radix(b, 16).map_err(serde::de::Error::custom))
        .collect()
}

pub fn serialize_as_hex_opt<S>(data: &Option<Vec<u8>>, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match data {
        Some(bytes) => serialize_as_hex(bytes, s),
        None => s.serialize_none(),
    }
}

pub fn deserialize_from_hex_opt<'de, D>(d: D) -> Result<Option<Vec<u8>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(d)?;
    match opt {
        None => Ok(None),
        Some(s) => s
            .split_whitespace()
            .map(|b| u8::from_str_radix(b, 16).map_err(serde::de::Error::custom))
            .collect::<Result<Vec<u8>, _>>()
            .map(Some),
    }
}
