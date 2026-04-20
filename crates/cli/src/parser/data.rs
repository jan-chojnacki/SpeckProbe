pub fn parse_chunks<const CHUNK: usize>(data: &str) -> Vec<u64> {
    data.as_bytes()
        .chunks(CHUNK)
        .map(|c| {
            let mut buf = [0u8; 8];
            buf[..c.len()].copy_from_slice(c);
            u64::from_le_bytes(buf)
        })
        .collect()
}

macro_rules! parse_data {
    ($name:ident, $chunk:literal) => {
        pub fn $name(data: &str) -> Vec<u64> {
            parse_chunks::<$chunk>(data)
        }
    };
}

parse_data!(parse_data_16, 2);
parse_data!(parse_data_24, 3);
parse_data!(parse_data_32, 4);
parse_data!(parse_data_48, 6);
parse_data!(parse_data_64, 8);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_speck32_64_plaintext() {
        assert_eq!(
            parse_data_16("Lite"),
            vec![0x0000_0000_0000_694c_u64, 0x0000_0000_0000_6574_u64]
        );
    }

    #[test]
    fn test_speck48_72_plaintext() {
        assert_eq!(
            parse_data_24("rally "),
            vec![0x0000_0000_006c_6172_u64, 0x0000_0000_0020_796c_u64]
        );
    }

    #[test]
    fn test_speck48_96_plaintext() {
        assert_eq!(
            parse_data_24("this m"),
            vec![0x0000_0000_0069_6874_u64, 0x0000_0000_006d_2073_u64]
        );
    }

    #[test]
    fn test_speck64_96_plaintext() {
        assert_eq!(
            parse_data_32("eans Fat"),
            vec![0x0000_0000_736e_6165_u64, 0x0000_0000_7461_4620_u64]
        );
    }

    #[test]
    fn test_speck64_128_plaintext() {
        assert_eq!(
            parse_data_32("-Cutter;"),
            vec![0x0000_0000_7475_432d_u64, 0x0000_0000_3b72_6574_u64]
        );
    }

    #[test]
    fn test_speck96_96_plaintext() {
        assert_eq!(
            parse_data_48(" usage, howe"),
            vec![0x0000_6567_6173_7520_u64, 0x0000_6577_6f68_202c_u64]
        );
    }

    #[test]
    fn test_speck96_144_plaintext() {
        assert_eq!(
            parse_data_48("ver, in time"),
            vec![0x0000_6920_2c72_6576_u64, 0x0000_656d_6974_206e_u64]
        );
    }

    #[test]
    fn test_speck128_128_plaintext() {
        assert_eq!(
            parse_data_64(" made it equival"),
            vec![0x7469_2065_6461_6d20_u64, 0x6c61_7669_7571_6520_u64]
        );
    }

    #[test]
    fn test_speck128_192_plaintext() {
        assert_eq!(
            parse_data_64("ent to Chief Har"),
            vec![0x4320_6f74_2074_6e65_u64, 0x7261_4820_6665_6968_u64]
        );
    }

    #[test]
    fn test_speck128_256_plaintext() {
        assert_eq!(
            parse_data_64("pooner. In those"),
            vec![0x202e_7265_6e6f_6f70_u64, 0x6573_6f68_7420_6e49_u64]
        );
    }
}
