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
