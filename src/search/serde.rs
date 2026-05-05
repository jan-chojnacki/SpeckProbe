use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use serde::Deserialize;

/// Serializes `[u64; 2]` pairs as a standard base64 string of little-endian bytes.
pub fn serialize_u64_pairs<S>(data: &[[u64; 2]], s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let bytes: Vec<u8> = data
        .iter()
        .flat_map(|[a, b]| a.to_le_bytes().into_iter().chain(b.to_le_bytes()))
        .collect();
    s.serialize_str(&STANDARD.encode(&bytes))
}

/// Deserializes a standard base64 string into `[u64; 2]` pairs (little-endian, 16 bytes each).
pub fn deserialize_u64_pairs<'de, D>(d: D) -> Result<Vec<[u64; 2]>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(d)?;
    let bytes = STANDARD.decode(&s).map_err(serde::de::Error::custom)?;

    if bytes.len() % 16 != 0 {
        return Err(serde::de::Error::custom(
            "data length must be a multiple of 16 bytes",
        ));
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

/// Serializes a byte slice as space-separated lowercase hex (e.g. `"0a 1b 2c"`).
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

/// Deserializes a space-separated hex string into a byte vector.
pub fn deserialize_from_hex<'de, D>(d: D) -> Result<Vec<u8>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(d)?;
    s.split_whitespace()
        .map(|b| u8::from_str_radix(b, 16).map_err(serde::de::Error::custom))
        .collect()
}

/// Serializes an optional `[u64; 2]` pair as a standard base64 string of little-endian bytes.
/// Uses the same 16-byte encoding as `serialize_u64_pairs` (one element).
pub fn serialize_u64_pair_opt<S>(data: &Option<[u64; 2]>, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match data {
        None => s.serialize_none(),
        Some([a, b]) => {
            let bytes: [u8; 16] = {
                let mut buf = [0u8; 16];
                buf[..8].copy_from_slice(&a.to_le_bytes());
                buf[8..].copy_from_slice(&b.to_le_bytes());
                buf
            };
            s.serialize_str(&STANDARD.encode(bytes))
        }
    }
}

/// Deserializes a standard base64 string (16 bytes little-endian) into `Option<[u64; 2]>`.
pub fn deserialize_u64_pair_opt<'de, D>(d: D) -> Result<Option<[u64; 2]>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(d)?;
    match opt {
        None => Ok(None),
        Some(s) => {
            let bytes = STANDARD.decode(&s).map_err(serde::de::Error::custom)?;
            if bytes.len() != 16 {
                return Err(serde::de::Error::custom(
                    "IV must be exactly 16 bytes (two little-endian u64 words)",
                ));
            }
            let a = u64::from_le_bytes(bytes[0..8].try_into().unwrap());
            let b = u64::from_le_bytes(bytes[8..16].try_into().unwrap());
            Ok(Some([a, b]))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct PairsWrapper {
        #[serde(
            serialize_with = "serialize_u64_pairs",
            deserialize_with = "deserialize_u64_pairs"
        )]
        data: Vec<[u64; 2]>,
    }

    #[derive(Serialize, Deserialize)]
    struct HexWrapper {
        #[serde(
            serialize_with = "serialize_as_hex",
            deserialize_with = "deserialize_from_hex"
        )]
        data: Vec<u8>,
    }

    #[derive(Serialize, Deserialize)]
    struct PairOptWrapper {
        #[serde(
            serialize_with = "serialize_u64_pair_opt",
            deserialize_with = "deserialize_u64_pair_opt"
        )]
        iv: Option<[u64; 2]>,
    }

    fn roundtrip_pairs(pairs: Vec<[u64; 2]>) -> Vec<[u64; 2]> {
        let json = serde_json::to_string(&PairsWrapper { data: pairs }).unwrap();
        serde_json::from_str::<PairsWrapper>(&json).unwrap().data
    }

    fn roundtrip_hex(bytes: Vec<u8>) -> Vec<u8> {
        let json = serde_json::to_string(&HexWrapper { data: bytes }).unwrap();
        serde_json::from_str::<HexWrapper>(&json).unwrap().data
    }

    fn roundtrip_pair_opt(iv: Option<[u64; 2]>) -> Option<[u64; 2]> {
        let json = serde_json::to_string(&PairOptWrapper { iv }).unwrap();
        serde_json::from_str::<PairOptWrapper>(&json).unwrap().iv
    }

    #[test]
    fn pairs_roundtrip_empty() {
        assert_eq!(roundtrip_pairs(vec![]), Vec::<[u64; 2]>::new());
    }

    #[rstest]
    #[case([0u64, 0u64])]
    #[case([1u64, 2u64])]
    #[case([u64::MAX, u64::MAX])]
    #[case([0x0102030405060708u64, 0x090a0b0c0d0e0fu64])]
    fn pairs_roundtrip_single(#[case] pair: [u64; 2]) {
        assert_eq!(roundtrip_pairs(vec![pair]), vec![pair]);
    }

    #[test]
    fn pairs_roundtrip_multiple() {
        let pairs = vec![[1u64, 2u64], [3u64, 4u64], [u64::MAX, 0u64]];
        assert_eq!(roundtrip_pairs(pairs.clone()), pairs);
    }

    #[test]
    fn pairs_serializes_little_endian() {
        let json = serde_json::to_string(&PairsWrapper {
            data: vec![[1u64, 0u64]],
        })
        .unwrap();
        let encoded: serde_json::Value = serde_json::from_str(&json).unwrap();
        let b64 = encoded["data"].as_str().unwrap();
        let bytes = STANDARD.decode(b64).unwrap();
        assert_eq!(bytes[..8], 1u64.to_le_bytes());
        assert_eq!(bytes[8..], 0u64.to_le_bytes());
    }

    #[rstest]
    #[case("not-base64!!")]
    #[case("dGVzdA==")]
    #[case("AQIDBA==")]
    fn pairs_deserialize_rejects_invalid(#[case] bad: &str) {
        let json = format!(r#"{{"data":"{bad}"}}"#);
        assert!(serde_json::from_str::<PairsWrapper>(&json).is_err());
    }

    #[test]
    fn hex_roundtrip_empty() {
        assert_eq!(roundtrip_hex(vec![]), Vec::<u8>::new());
    }

    #[rstest]
    #[case(vec![0x00u8])]
    #[case(vec![0xffu8])]
    #[case(vec![0x0au8, 0x1bu8, 0x2cu8])]
    #[case(vec![0u8, 1, 2, 3, 255])]
    fn hex_roundtrip(#[case] bytes: Vec<u8>) {
        assert_eq!(roundtrip_hex(bytes.clone()), bytes);
    }

    #[test]
    fn hex_serializes_space_separated_lowercase() {
        let json = serde_json::to_string(&HexWrapper {
            data: vec![0x0au8, 0x1bu8, 0x2cu8],
        })
        .unwrap();
        let val: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(val["data"].as_str().unwrap(), "0a 1b 2c");
    }

    #[rstest]
    #[case("zz")]
    #[case("0g")]
    #[case("gg gg")]
    fn hex_deserialize_rejects_invalid(#[case] bad: &str) {
        let json = format!(r#"{{"data":"{bad}"}}"#);
        assert!(serde_json::from_str::<HexWrapper>(&json).is_err());
    }

    #[test]
    fn pair_opt_roundtrip_none() {
        assert_eq!(roundtrip_pair_opt(None), None);
    }

    #[rstest]
    #[case([0u64, 0u64])]
    #[case([1u64, 2u64])]
    #[case([u64::MAX, u64::MAX])]
    fn pair_opt_roundtrip_some(#[case] pair: [u64; 2]) {
        assert_eq!(roundtrip_pair_opt(Some(pair)), Some(pair));
    }

    #[test]
    fn pair_opt_serializes_little_endian() {
        let json = serde_json::to_string(&PairOptWrapper {
            iv: Some([1u64, 2u64]),
        })
        .unwrap();
        let val: serde_json::Value = serde_json::from_str(&json).unwrap();
        let b64 = val["iv"].as_str().unwrap();
        let bytes = STANDARD.decode(b64).unwrap();
        assert_eq!(bytes.len(), 16);
        assert_eq!(bytes[..8], 1u64.to_le_bytes());
        assert_eq!(bytes[8..], 2u64.to_le_bytes());
    }

    #[rstest]
    #[case("not-base64!!")]
    #[case("AQID")]
    #[case("dGVzdA==")]
    fn pair_opt_deserialize_rejects_wrong_length(#[case] bad: &str) {
        let json = format!(r#"{{"iv":"{bad}"}}"#);
        assert!(serde_json::from_str::<PairOptWrapper>(&json).is_err());
    }
}
