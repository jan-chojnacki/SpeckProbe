use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ParseKeyError {
    #[error("Niedozwolony znak: '{0}'")]
    InvalidChar(char),

    #[error("Nieparzysta liczba znaków hex")]
    OddNibbles,

    #[error("Oczekiwano {expected} par, znaleziono {got}")]
    WrongPairCount { expected: usize, got: usize },
}

pub fn validate_key(key: &str, expected_pairs: usize) -> Result<(), ParseKeyError> {
    let stripped: String = key.chars().filter(|c| !c.is_whitespace()).collect();

    if let Some(bad) = stripped.chars().find(|c| !c.is_ascii_hexdigit()) {
        return Err(ParseKeyError::InvalidChar(bad));
    }

    if stripped.len() % 2 != 0 {
        return Err(ParseKeyError::OddNibbles);
    }

    let got = stripped.len() / 2;
    if got != expected_pairs {
        return Err(ParseKeyError::WrongPairCount {
            expected: expected_pairs,
            got,
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_valid_no_spaces() {
        assert_eq!(validate_key("deadbeef", 4), Ok(()));
    }

    #[test]
    fn test_valid_spaces_between_pairs() {
        assert_eq!(validate_key("de ad be ef", 4), Ok(()));
    }

    #[test]
    fn test_valid_spaces_between_groups() {
        assert_eq!(validate_key("dead beef", 4), Ok(()));
    }

    #[test]
    fn test_valid_uppercase() {
        assert_eq!(validate_key("DEADBEEF", 4), Ok(()));
    }

    #[test]
    fn test_valid_mixed_case() {
        assert_eq!(validate_key("De aD bE Ef", 4), Ok(()));
    }

    #[test]
    fn test_valid_leading_trailing_spaces() {
        assert_eq!(validate_key("  de ad be ef  ", 4), Ok(()));
    }

    #[test]
    fn test_valid_single_pair() {
        assert_eq!(validate_key("ff", 1), Ok(()));
    }

    #[test]
    fn test_invalid_char_letter() {
        assert_eq!(
            validate_key("de xd be ef", 4),
            Err(ParseKeyError::InvalidChar('x'))
        );
    }

    #[test]
    fn test_invalid_char_g() {
        assert_eq!(
            validate_key("deadbeg", 4),
            Err(ParseKeyError::InvalidChar('g'))
        );
    }

    #[test]
    fn test_invalid_char_dash() {
        assert_eq!(
            validate_key("de-ad", 2),
            Err(ParseKeyError::InvalidChar('-'))
        );
    }

    #[test]
    fn test_invalid_char_checked_before_odd() {
        assert_eq!(validate_key("dex", 2), Err(ParseKeyError::InvalidChar('x')));
    }

    #[test]
    fn test_odd_nibbles_single() {
        assert_eq!(validate_key("d", 1), Err(ParseKeyError::OddNibbles));
    }

    #[test]
    fn test_odd_nibbles_after_stripping() {
        assert_eq!(validate_key("de a", 2), Err(ParseKeyError::OddNibbles));
    }

    #[test]
    fn test_too_few_pairs() {
        assert_eq!(
            validate_key("de ad be", 4),
            Err(ParseKeyError::WrongPairCount {
                expected: 4,
                got: 3
            })
        );
    }

    #[test]
    fn test_too_many_pairs() {
        assert_eq!(
            validate_key("de ad be ef ff", 4),
            Err(ParseKeyError::WrongPairCount {
                expected: 4,
                got: 5
            })
        );
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(
            validate_key("", 4),
            Err(ParseKeyError::WrongPairCount {
                expected: 4,
                got: 0
            })
        );
    }

    #[test]
    fn test_only_spaces() {
        assert_eq!(
            validate_key("     ", 4),
            Err(ParseKeyError::WrongPairCount {
                expected: 4,
                got: 0
            })
        );
    }
}
