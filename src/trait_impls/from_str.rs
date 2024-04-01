use crate::{UncToken, UncTokenError, ONE_UNC};

impl std::str::FromStr for UncToken {
    type Err = UncTokenError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uppercase_s = s.trim().to_ascii_uppercase();
        let (value, unit) = uppercase_s.split_at(
            s.find(|c: char| c.is_ascii_alphabetic())
                .ok_or_else(|| UncTokenError::InvalidTokenUnit(s.to_owned()))?,
        );
        let unit_precision = match unit {
            "YN" | "YUNC" | "YOCTOUNC" => 1,
            "UNC" | "N" => ONE_UNC,
            _ => return Err(UncTokenError::InvalidTokenUnit(s.to_owned())),
        };
        Ok(UncToken::from_attounc(
            crate::utils::parse_decimal_number(value.trim(), unit_precision)
                .map_err(UncTokenError::InvalidTokensAmount)?,
        ))
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::{DecimalNumberParsingError, UncToken, UncTokenError};

    #[test]
    fn parse_decimal_number() {
        let data = "0.123456 unc";
        let gas: Result<UncToken, UncTokenError> = FromStr::from_str(data);
        assert_eq!(
            gas.unwrap(),
            UncToken::from_attounc(123456000000000000000000)
        );
    }
    #[test]
    fn parse_number_with_decimal_part() {
        let data = "11.123456 unc";
        let gas: Result<UncToken, UncTokenError> = FromStr::from_str(data);
        assert_eq!(
            gas.unwrap(),
            UncToken::from_attounc(11123456000000000000000000)
        );
    }

    #[test]
    fn parse_atto_number() {
        let data = "123456 YN";
        let gas: Result<UncToken, UncTokenError> = FromStr::from_str(data);
        assert_eq!(gas.unwrap(), UncToken::from_attounc(123456));
    }

    #[test]
    fn doubledot() {
        let data = "1.1.1 Near";
        let gas: Result<UncToken, UncTokenError> = FromStr::from_str(data);
        assert_eq!(
            gas,
            Err(UncTokenError::InvalidTokensAmount(
                DecimalNumberParsingError::InvalidNumber("1.1.1".to_owned())
            ))
        )
    }

    #[test]
    fn space_after_dot() {
        let data = "1. 0 unc";
        let gas: Result<UncToken, UncTokenError> = FromStr::from_str(data);
        assert_eq!(
            gas,
            Err(UncTokenError::InvalidTokensAmount(
                DecimalNumberParsingError::InvalidNumber("1. 0".to_owned())
            ))
        )
    }

    #[test]
    fn incorect_currency() {
        let data = "0 pas";
        let gas: Result<UncToken, UncTokenError> = FromStr::from_str(data);
        assert_eq!(gas, Err(UncTokenError::InvalidTokenUnit(data.to_owned())))
    }

    #[test]
    fn without_currency() {
        let data = "0";
        let gas: Result<UncToken, UncTokenError> = FromStr::from_str(data);
        assert_eq!(gas, Err(UncTokenError::InvalidTokenUnit("0".to_owned())))
    }

    #[test]
    fn invalid_whole() {
        let data = "-1 Near";
        let gas: Result<UncToken, UncTokenError> = FromStr::from_str(data);
        assert_eq!(
            gas,
            Err(UncTokenError::InvalidTokensAmount(
                DecimalNumberParsingError::InvalidNumber("-1".to_owned())
            ))
        )
    }

    #[test]
    fn test_from_str_f64_gas_without_int() {
        let near_gas = UncToken::from_str(".055 ynear").unwrap_err();
        assert_eq!(
            near_gas,
            UncTokenError::InvalidTokensAmount(DecimalNumberParsingError::InvalidNumber(
                ".055".to_string()
            ))
        );
    }

    #[test]
    fn test_from_str_without_unit() {
        let near_gas = UncToken::from_str("100").unwrap_err();
        assert_eq!(
            near_gas,
            UncTokenError::InvalidTokenUnit("100".to_string())
        );
    }

    #[test]
    fn test_from_str_incorrect_unit() {
        let near_gas = UncToken::from_str("100 UAH").unwrap_err();
        assert_eq!(
            near_gas,
            UncTokenError::InvalidTokenUnit("100 UAH".to_string())
        );
    }

    #[test]
    fn test_from_str_invalid_double_dot() {
        let near_gas = UncToken::from_str("100.55.").unwrap_err();
        assert_eq!(
            near_gas,
            UncTokenError::InvalidTokenUnit("100.55.".to_string())
        );
    }

    #[test]
    fn test_from_str_large_fractional_part() {
        let near_gas = UncToken::from_str("100.1111122222333 ynear").unwrap_err(); // 13 digits after "."
        assert_eq!(
            near_gas,
            UncTokenError::InvalidTokensAmount(DecimalNumberParsingError::LongFractional(
                "1111122222333".to_string()
            ))
        );
    }
}
