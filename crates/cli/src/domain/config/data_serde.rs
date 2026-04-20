use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use serde::Deserialize;

pub fn serialize_u64_pairs<S>(data: &Vec<[u64; 2]>, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let bytes: Vec<u8> = data
        .iter()
        .flat_map(|[a, b]| a.to_le_bytes().into_iter().chain(b.to_le_bytes()))
        .collect();
    s.serialize_str(&STANDARD.encode(&bytes))
}

pub fn deserialize_u64_pairs<'de, D>(d: D) -> Result<Vec<[u64; 2]>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(d)?;
    let bytes = STANDARD.decode(&s).map_err(serde::de::Error::custom)?;

    if bytes.len() % 16 != 0 {
        return Err(serde::de::Error::custom("invalid length"));
    }

    Ok(bytes
        .chunks_exact(16)
        .map(|c| {
            let a = u64::from_le_bytes(c[0..8].try_into().unwrap());
            let b = u64::from_le_bytes(c[8..16].try_into().unwrap());
            [a, b]
        })
        .collect())
}
