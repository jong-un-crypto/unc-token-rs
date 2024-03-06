//! A `UncToken` type to represent a value of Near.
//!
//! Each `UncTokens` is composed of a floating point number of tokens where each integer unit is equal to one yocto-Near.
//! `UncToken` is implementing the common trait `FromStr`. Also, have utils function to parse from `str` into `u128`.
//!
//! # Examples
//! ```
//! use unc_token::UncToken;
//!
//! let one_near = UncToken::from_yoctonear(10_u128.pow(24));
//! assert_eq!(one_near, UncToken::from_unc(1));
//! assert_eq!(one_near, UncToken::from_millinear(1000));
//! ```
//!
//! # Crate features
//!
//! * **borsh** (optional) -
//!   When enabled allows `UncToken` to serialized and deserialized by `borsh`.
//!
//! * **serde** (optional) -
//!   When enabled allows `UncToken` to serialized and deserialized by `serde`.
//!
//! * **schemars** (optional) -
//!  Implements `schemars::JsonSchema` for `UncToken`.
//!
//! * **interactive-clap** (optional) -
//!  Implements `interactive_clap::ToCli` for `UncToken`.
mod error;

mod utils;

mod trait_impls;

pub use self::error::UncTokenError;
pub use self::utils::DecimalNumberParsingError;

#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshDeserialize, borsh::BorshSerialize)
)]
#[cfg_attr(feature = "abi", derive(borsh::BorshSchema))]
#[repr(transparent)]
pub struct UncToken {
    inner: u128,
}

const ONE_UNC: u128 = 10_u128.pow(24);
const ONE_MILLIUNC: u128 = 10_u128.pow(21);

impl UncToken {
    /// `from_yoctonear` is a function that takes value by a number of yocto-unc.
    /// # Examples
    /// ```
    /// use unc_token::UncToken;
    /// assert_eq!( UncToken::from_yoctonear(10u128.pow(21)), UncToken::from_millinear(1))
    /// ```
    pub const fn from_yoctonear(inner: u128) -> Self {
        Self { inner }
    }

    /// `from_millinear` is a function that takes value by a number of mili-unc and converts it to an equivalent to the yocto-unc.
    /// # Examples
    /// ```
    /// use unc_token::UncToken;
    /// assert_eq!(UncToken::from_millinear(1), UncToken::from_yoctonear(10u128.pow(21)))
    /// ```
    pub const fn from_millinear(inner: u128) -> Self {
        Self {
            inner: inner * ONE_MILLIUNC,
        }
    }

    /// `from_unc` is a function that takes value by a number of unc and converts it to an equivalent to the yocto-unc.
    /// # Examples
    /// ```
    /// use unc_token::UncToken;
    /// assert_eq!(UncToken::from_unc(1), UncToken::from_yoctonear(10u128.pow(24)))
    /// ```
    pub const fn from_unc(inner: u128) -> Self {
        Self {
            inner: inner * ONE_UNC,
        }
    }

    /// `as_near` is a function that converts number of yocto-unc to an equivalent to the unc.
    /// # Examples
    /// ```
    /// use unc_token::UncToken;
    /// assert_eq!(UncToken::from_yoctonear(10u128.pow(24)).as_near(), 1)
    /// ```
    pub const fn as_near(&self) -> u128 {
        self.inner / ONE_UNC
    }

    /// `as_millinear` is a function that converts number of yocto-unc to an equivalent to the mili-unc.
    /// # Examples
    /// ```
    /// use unc_token::UncToken;
    /// assert_eq!(UncToken::from_yoctonear(10u128.pow(21)).as_millinear(), 1)
    /// ```
    pub const fn as_millinear(&self) -> u128 {
        self.inner / ONE_MILLIUNC
    }

    /// `as_yoctonear` is a function that shows a number of yocto-unc.
    /// # Examples
    /// ```
    /// use unc_token::UncToken;
    /// assert_eq!(UncToken::from_yoctonear(10).as_yoctonear(), 10)
    /// ```
    pub const fn as_yoctonear(&self) -> u128 {
        self.inner
    }

    /// `is_zero` is a boolian function that checks `UncToken`
    /// if a `UncToken` inner is zero, returns true.
    /// # Examples
    /// ```
    /// use unc_token::UncToken;
    /// assert_eq!(UncToken::from_yoctonear(0).is_zero(), true)
    /// ```
    pub const fn is_zero(&self) -> bool {
        self.inner == 0
    }

    /// Checked integer addition. Computes self + rhs, returning None if overflow occurred.
    ///
    /// # Examples
    /// ```
    /// use unc_token::UncToken;
    /// use std::u128;
    /// assert_eq!(UncToken::from_yoctonear(u128::MAX -2).checked_add(UncToken::from_yoctonear(2)), Some(UncToken::from_yoctonear(u128::MAX)));
    /// assert_eq!(UncToken::from_yoctonear(u128::MAX -2).checked_add(UncToken::from_yoctonear(3)), None);
    /// ```
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        if let Some(unc) = self.as_yoctonear().checked_add(rhs.as_yoctonear()) {
            Some(Self::from_yoctonear(unc))
        } else {
            None
        }
    }

    /// Checked integer subtraction. Computes self - rhs, returning None if overflow occurred.
    ///
    /// # Examples
    /// ```
    /// use unc_token::UncToken;
    /// assert_eq!(UncToken::from_yoctonear(2).checked_sub(UncToken::from_yoctonear(2)), Some(UncToken::from_yoctonear(0)));
    /// assert_eq!(UncToken::from_yoctonear(2).checked_sub(UncToken::from_yoctonear(3)), None);
    /// ```
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        if let Some(unc) = self.as_yoctonear().checked_sub(rhs.as_yoctonear()) {
            Some(Self::from_yoctonear(unc))
        } else {
            None
        }
    }

    /// Checked integer multiplication. Computes self * rhs, returning None if overflow occurred.
    ///
    /// # Examples
    /// ```
    /// use unc_token::UncToken;
    /// use std::u128;
    /// assert_eq!(UncToken::from_yoctonear(2).checked_mul(2), Some(UncToken::from_yoctonear(4)));
    /// assert_eq!(UncToken::from_yoctonear(u128::MAX).checked_mul(2), None)
    pub const fn checked_mul(self, rhs: u128) -> Option<Self> {
        if let Some(unc) = self.as_yoctonear().checked_mul(rhs) {
            Some(Self::from_yoctonear(unc))
        } else {
            None
        }
    }

    /// Checked integer division. Computes self / rhs, returning None if rhs == 0.
    ///
    /// # Examples
    /// ```
    /// use unc_token::UncToken;
    /// assert_eq!(UncToken::from_yoctonear(10).checked_div(2), Some(UncToken::from_yoctonear(5)));
    /// assert_eq!(UncToken::from_yoctonear(2).checked_div(0), None);
    /// ```
    pub const fn checked_div(self, rhs: u128) -> Option<Self> {
        if let Some(unc) = self.as_yoctonear().checked_div(rhs) {
            Some(Self::from_yoctonear(unc))
        } else {
            None
        }
    }

    /// Saturating integer addition. Computes self + rhs, saturating at the numeric bounds instead of overflowing.
    ///
    /// # Examples
    /// ```
    /// use unc_token::UncToken;
    /// assert_eq!(UncToken::from_yoctonear(5).saturating_add(UncToken::from_yoctonear(5)), UncToken::from_yoctonear(10));
    /// assert_eq!(UncToken::from_yoctonear(u128::MAX).saturating_add(UncToken::from_yoctonear(1)), UncToken::from_yoctonear(u128::MAX));
    /// ```
    pub const fn saturating_add(self, rhs: Self) -> Self {
        UncToken::from_yoctonear(self.as_yoctonear().saturating_add(rhs.as_yoctonear()))
    }

    /// Saturating integer subtraction. Computes self - rhs, saturating at the numeric bounds instead of overflowing.
    ///
    /// # Examples
    /// ```
    /// use unc_token::UncToken;
    /// assert_eq!(UncToken::from_yoctonear(5).saturating_sub(UncToken::from_yoctonear(2)), UncToken::from_yoctonear(3));
    /// assert_eq!(UncToken::from_yoctonear(1).saturating_sub(UncToken::from_yoctonear(2)), UncToken::from_yoctonear(0));
    /// ```
    pub const fn saturating_sub(self, rhs: Self) -> Self {
        UncToken::from_yoctonear(self.as_yoctonear().saturating_sub(rhs.as_yoctonear()))
    }

    /// Saturating integer multiplication. Computes self * rhs, saturating at the numeric bounds instead of overflowing.
    ///
    /// # Examples
    /// ```
    /// use unc_token::UncToken;
    /// use std::u128;
    /// assert_eq!(UncToken::from_yoctonear(2).saturating_mul(5), UncToken::from_yoctonear(10));
    /// assert_eq!(UncToken::from_yoctonear(u128::MAX).saturating_mul(2), UncToken::from_yoctonear(u128::MAX));
    /// ```
    pub const fn saturating_mul(self, rhs: u128) -> Self {
        UncToken::from_yoctonear(self.as_yoctonear().saturating_mul(rhs))
    }

    /// Saturating integer division. Computes self / rhs, saturating at the numeric bounds instead of overflowing.
    ///
    /// # Examples
    /// ```
    /// use unc_token::UncToken;
    /// assert_eq!(UncToken::from_yoctonear(10).saturating_div(2), UncToken::from_yoctonear(5));
    /// assert_eq!(UncToken::from_yoctonear(10).saturating_div(0), UncToken::from_yoctonear(0))
    /// ```
    pub const fn saturating_div(self, rhs: u128) -> Self {
        if rhs == 0 {
            return UncToken::from_yoctonear(0);
        }
        UncToken::from_yoctonear(self.as_yoctonear().saturating_div(rhs))
    }
}

#[cfg(test)]
mod test {
    use crate::UncToken;

    #[test]
    fn checked_add_tokens() {
        let tokens = UncToken::from_yoctonear(u128::MAX - 3);
        let any_tokens = UncToken::from_yoctonear(3);
        let more_tokens = UncToken::from_yoctonear(4);
        assert_eq!(
            tokens.checked_add(any_tokens),
            Some(UncToken::from_yoctonear(u128::MAX))
        );
        assert_eq!(tokens.checked_add(more_tokens), None);
    }

    #[test]
    fn checked_sub_tokens() {
        let tokens = UncToken::from_yoctonear(3);
        let any_tokens = UncToken::from_yoctonear(1);
        let more_tokens = UncToken::from_yoctonear(4);
        assert_eq!(
            tokens.checked_sub(any_tokens),
            Some(UncToken::from_yoctonear(2))
        );
        assert_eq!(tokens.checked_sub(more_tokens), None);
    }

    #[test]
    fn checked_mul_tokens() {
        let tokens = UncToken::from_yoctonear(u128::MAX / 10);
        assert_eq!(
            tokens.checked_mul(10),
            Some(UncToken::from_yoctonear(u128::MAX / 10 * 10))
        );
        assert_eq!(tokens.checked_mul(11), None);
    }

    #[test]
    fn checked_div_tokens() {
        let tokens = UncToken::from_yoctonear(10);
        assert_eq!(tokens.checked_div(2), Some(UncToken::from_yoctonear(5)));
        assert_eq!(tokens.checked_div(11), Some(UncToken::from_yoctonear(0)));
        assert_eq!(tokens.checked_div(0), None);
    }

    #[test]
    fn saturating_add_tokens() {
        let tokens = UncToken::from_yoctonear(100);
        let added_tokens = UncToken::from_yoctonear(1);
        let another_tokens = UncToken::from_yoctonear(u128::MAX);
        assert_eq!(
            tokens.saturating_add(added_tokens.clone()),
            UncToken::from_yoctonear(101)
        );
        assert_eq!(
            another_tokens.saturating_add(added_tokens),
            UncToken::from_yoctonear(u128::MAX)
        );
    }

    #[test]
    fn saturating_sub_tokens() {
        let tokens = UncToken::from_yoctonear(100);
        let rhs_tokens = UncToken::from_yoctonear(1);
        let another_tokens = UncToken::from_yoctonear(u128::MIN);
        assert_eq!(
            tokens.saturating_sub(rhs_tokens.clone()),
            UncToken::from_yoctonear(99)
        );
        assert_eq!(
            another_tokens.saturating_sub(rhs_tokens),
            UncToken::from_yoctonear(u128::MIN)
        );
    }

    #[test]
    fn saturating_mul_tokens() {
        let tokens = UncToken::from_yoctonear(2);
        let rhs = 10;
        let another_tokens = u128::MAX;
        assert_eq!(tokens.saturating_mul(rhs), UncToken::from_yoctonear(20));
        assert_eq!(
            tokens.saturating_mul(another_tokens),
            UncToken::from_yoctonear(u128::MAX)
        );
    }

    #[test]
    fn saturating_div_tokens() {
        let tokens = UncToken::from_yoctonear(10);
        let rhs = 2;
        let another_tokens = 20;
        assert_eq!(tokens.saturating_div(rhs), UncToken::from_yoctonear(5));
        assert_eq!(
            tokens.saturating_div(another_tokens),
            UncToken::from_yoctonear(0)
        );
    }
}
