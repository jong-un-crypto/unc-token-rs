use crate::{UncToken, ONE_MILLIUNC};

/// UncToken Display implementation rounds up the token amount to the relevant precision point.
/// There are 4 breakpoints:
/// 1. exactly 0 UNC
/// 2. <0.001 UNC
/// 3. 0.001 - 0.999 UNC (uses 3 digits after the floating point)
/// 4. >1 UNC (uses 2 digits after the floating point)
impl std::fmt::Display for UncToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if *self == UncToken::from_yoctounc(0) {
            write!(f, "0 UNC")
        } else if *self < UncToken::from_milliunc(1) {
            write!(f, "<0.001 UNC")
        } else if *self <= UncToken::from_milliunc(999) {
            let millinear_rounded_up =
                self.as_yoctounc().saturating_add(ONE_MILLIUNC - 1) / ONE_MILLIUNC;
            write!(f, "0.{:03} UNC", millinear_rounded_up)
        } else {
            let near_rounded_up =
                self.as_yoctounc().saturating_add(10 * ONE_MILLIUNC - 1) / ONE_MILLIUNC / 10;
            write!(
                f,
                "{}.{:02} UNC",
                near_rounded_up / 100,
                near_rounded_up % 100
            )
        }
    }
}

#[cfg(test)]
mod test {
    use crate::UncToken;

    #[test]
    fn test_display() {
        for (unc_tokens, expected_display) in [
            (UncToken::from_yoctounc(0), "0 UNC"),
            (UncToken::from_yoctounc(1), "<0.001 UNC"),
            (UncToken::from_yoctounc(10u128.pow(21) - 1), "<0.001 UNC"),
            (UncToken::from_yoctounc(10u128.pow(21)), "0.001 UNC"),
            (UncToken::from_yoctounc(10u128.pow(21) + 1), "0.002 UNC"),
            (UncToken::from_yoctounc(10u128.pow(21) * 2), "0.002 UNC"),
            (
                UncToken::from_yoctounc(10u128.pow(21) * 200),
                "0.200 UNC",
            ),
            (
                UncToken::from_yoctounc(10u128.pow(21) * 999),
                "0.999 UNC",
            ),
            (
                UncToken::from_yoctounc(10u128.pow(21) * 999 + 1),
                "1.00 UNC",
            ),
            (UncToken::from_yoctounc(10u128.pow(24) - 1), "1.00 UNC"),
            (UncToken::from_yoctounc(10u128.pow(24)), "1.00 UNC"),
            (UncToken::from_yoctounc(10u128.pow(24) + 1), "1.01 UNC"),
            (
                UncToken::from_yoctounc(10u128.pow(21) * 1234),
                "1.24 UNC",
            ),
            (
                UncToken::from_yoctounc(10u128.pow(21) * 1500),
                "1.50 UNC",
            ),
            (
                UncToken::from_yoctounc(10u128.pow(21) * 10000),
                "10.00 UNC",
            ),
            (
                UncToken::from_yoctounc(10u128.pow(21) * 10500),
                "10.50 UNC",
            ),
            (
                UncToken::from_yoctounc(10u128.pow(21) * 100000 - 1),
                "100.00 UNC",
            ),
            (
                UncToken::from_yoctounc(10u128.pow(21) * 100000),
                "100.00 UNC",
            ),
            (
                UncToken::from_yoctounc(10u128.pow(21) * 100500),
                "100.50 UNC",
            ),
            (
                UncToken::from_yoctounc(10u128.pow(21) * 100000000),
                "100000.00 UNC",
            ),
            (
                UncToken::from_yoctounc(10u128.pow(21) * 100000500),
                "100000.50 UNC",
            ),
        ] {
            assert_eq!(
                unc_tokens.to_string(),
                expected_display,
                "tokens: {}",
                unc_tokens.as_yoctounc()
            );
        }
    }
}
