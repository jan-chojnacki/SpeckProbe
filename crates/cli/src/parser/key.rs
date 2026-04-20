pub fn parse_key(key: &str) -> Vec<u8> {
    key.split_whitespace()
        .flat_map(|c| {
            c.as_bytes()
                .chunks(2)
                .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_spaces() {
        assert_eq!(parse_key("deadbeef"), vec![0xde, 0xad, 0xbe, 0xef]);
    }

    #[test]
    fn test_spaces_between_pairs() {
        assert_eq!(parse_key("de ad be ef"), vec![0xde, 0xad, 0xbe, 0xef]);
    }

    #[test]
    fn test_spaces_between_groups() {
        assert_eq!(parse_key("dead beef"), vec![0xde, 0xad, 0xbe, 0xef]);
    }

    #[test]
    fn test_single_byte() {
        assert_eq!(parse_key("ff"), vec![0xff]);
    }

    #[test]
    fn test_zeros() {
        assert_eq!(parse_key("00 00 00"), vec![0x00, 0x00, 0x00]);
    }

    #[test]
    fn test_uppercase() {
        assert_eq!(parse_key("DE AD BE EF"), vec![0xde, 0xad, 0xbe, 0xef]);
    }

    #[test]
    fn test_mixed_case() {
        assert_eq!(parse_key("De aD bE Ef"), vec![0xde, 0xad, 0xbe, 0xef]);
    }

    #[test]
    fn test_leading_trailing_spaces() {
        assert_eq!(parse_key("  de ad  "), vec![0xde, 0xad]);
    }

    #[test]
    fn test_all_bytes() {
        let input = (0u8..=255)
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join(" ");
        let expected: Vec<u8> = (0u8..=255).collect();
        assert_eq!(parse_key(&input), expected);
    }
}
