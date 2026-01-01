pub use reqwest::header::{HeaderName, HeaderValue};
pub use reqwest::Client;
pub use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
pub use rust_decimal::Decimal;
pub use rust_decimal_macros::dec;
pub use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub trait IntoDecimal {
    fn into_decimal(self) -> Decimal;
    fn try_into_decimal(self) -> Result<Decimal, String>;
}

impl IntoDecimal for Decimal {
    fn into_decimal(self) -> Decimal {
        self
    }

    fn try_into_decimal(self) -> Result<Decimal, String> {
        Ok(self)
    }
}

impl IntoDecimal for &Decimal {
    fn into_decimal(self) -> Decimal {
        *self
    }

    fn try_into_decimal(self) -> Result<Decimal, String> {
        Ok(*self)
    }
}

macro_rules! impl_into_decimal_int {
    ($($t:ty),*) => {
        $(
            impl IntoDecimal for $t {
                fn into_decimal(self) -> Decimal {
                    Decimal::from(self)
                }

                fn try_into_decimal(self) -> Result<Decimal, String> {
                    Ok(Decimal::from(self))
                }
            }
        )*
    };
}

impl_into_decimal_int!(u8, i8, u16, i16, u32, i32, u64, i64, isize, usize);

impl IntoDecimal for f32 {
    fn into_decimal(self) -> Decimal {
        self.try_into_decimal().unwrap_or(Decimal::ZERO)
    }

    fn try_into_decimal(self) -> Result<Decimal, String> {
        Decimal::from_f32(self).ok_or_else(|| format!("Invalid float value: {}", self))
    }
}

impl IntoDecimal for f64 {
    fn into_decimal(self) -> Decimal {
        self.try_into_decimal().unwrap_or(Decimal::ZERO)
    }

    fn try_into_decimal(self) -> Result<Decimal, String> {
        Decimal::from_f64(self).ok_or_else(|| format!("Invalid float value: {}", self))
    }
}

impl IntoDecimal for &str {
    fn into_decimal(self) -> Decimal {
        self.try_into_decimal().unwrap_or(Decimal::ZERO)
    }

    fn try_into_decimal(self) -> Result<Decimal, String> {
        Decimal::from_str(self).map_err(|e| format!("Invalid decimal string '{}': {}", self, e))
    }
}

impl IntoDecimal for String {
    fn into_decimal(self) -> Decimal {
        self.try_into_decimal().unwrap_or(Decimal::ZERO)
    }

    fn try_into_decimal(self) -> Result<Decimal, String> {
        Decimal::from_str(&self).map_err(|e| format!("Invalid decimal string '{}': {}", self, e))
    }
}

impl IntoDecimal for &String {
    fn into_decimal(self) -> Decimal {
        self.try_into_decimal().unwrap_or(Decimal::ZERO)
    }

    fn try_into_decimal(self) -> Result<Decimal, String> {
        Decimal::from_str(self).map_err(|e| format!("Invalid decimal string '{}': {}", self, e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal_conversions() {
        let decimal = dec!(123.45);
        assert_eq!(decimal.into_decimal(), dec!(123.45));
        assert!(decimal.try_into_decimal().is_ok());
    }

    #[test]
    fn test_decimal_ref_conversions() {
        let decimal = dec!(123.45);
        assert_eq!((&decimal).into_decimal(), dec!(123.45));
        assert!((&decimal).try_into_decimal().is_ok());
    }

    #[test]
    fn test_integer_conversions() {
        assert_eq!(42u8.into_decimal(), dec!(42));
        assert_eq!((-100i32).into_decimal(), dec!(-100));
        assert_eq!(999u64.into_decimal(), dec!(999));

        assert!(42u8.try_into_decimal().is_ok());
        assert!((-100i32).try_into_decimal().is_ok());
        assert!(999u64.try_into_decimal().is_ok());
    }

    #[test]
    fn test_float_conversions() {
        assert_eq!(3.14f32.into_decimal(), dec!(3.14));
        assert!(3.14f32.try_into_decimal().is_ok());

        assert_eq!(2.718281828f64.into_decimal(), dec!(2.718281828));
        assert!(2.718281828f64.try_into_decimal().is_ok());

        assert!(f32::NAN.try_into_decimal().is_err());
        assert!(f32::INFINITY.try_into_decimal().is_err());
        assert!(f64::NAN.try_into_decimal().is_err());
        assert!(f64::INFINITY.try_into_decimal().is_err());

        assert_eq!(f32::NAN.into_decimal(), Decimal::ZERO);
        assert_eq!(f64::INFINITY.into_decimal(), Decimal::ZERO);
    }

    #[test]
    fn test_string_conversions() {
        let valid_str = "123.45";
        let valid_string = String::from("678.90");
        let valid_string_ref = &String::from("42.0");

        assert_eq!(valid_str.into_decimal(), dec!(123.45));
        assert_eq!(valid_string.clone().into_decimal(), dec!(678.90));
        assert_eq!(valid_string_ref.into_decimal(), dec!(42.0));

        assert!(valid_str.try_into_decimal().is_ok());
        assert!(valid_string.try_into_decimal().is_ok());
        assert!(valid_string_ref.try_into_decimal().is_ok());

        let invalid_str = "not_a_number";
        let invalid_string = String::from("abc123");
        let invalid_string_ref = &String::from("invalid");

        assert!(invalid_str.try_into_decimal().is_err());
        assert!(invalid_string.clone().try_into_decimal().is_err());
        assert!(invalid_string_ref.try_into_decimal().is_err());

        assert_eq!(invalid_str.into_decimal(), Decimal::ZERO);
        assert_eq!(invalid_string.into_decimal(), Decimal::ZERO);
        assert_eq!(invalid_string_ref.into_decimal(), Decimal::ZERO);
    }

    #[test]
    fn test_empty_string_conversion() {
        let empty_str = "";
        let empty_string = String::new();
        let empty_string_ref = &String::new();

        assert!(empty_str.try_into_decimal().is_err());
        assert!(empty_string.clone().try_into_decimal().is_err());
        assert!(empty_string_ref.try_into_decimal().is_err());

        assert_eq!(empty_str.into_decimal(), Decimal::ZERO);
        assert_eq!(empty_string.into_decimal(), Decimal::ZERO);
        assert_eq!(empty_string_ref.into_decimal(), Decimal::ZERO);
    }

    #[test]
    fn test_edge_case_numbers() {
        assert_eq!(0.into_decimal(), Decimal::ZERO);
        assert_eq!(0.0f32.into_decimal(), Decimal::ZERO);
        assert_eq!("0".into_decimal(), Decimal::ZERO);

        assert_eq!((-42i32).into_decimal(), dec!(-42));
        assert_eq!((-3.14f64).into_decimal(), dec!(-3.14));
        assert_eq!("-123.45".into_decimal(), dec!(-123.45));

        assert_eq!(0.000001f32.into_decimal(), dec!(0.000001));
        assert_eq!(999999999.0f64.into_decimal(), dec!(999999999.0));
    }

    #[test]
    fn test_error_messages() {
        let error = "invalid".try_into_decimal().unwrap_err();
        assert!(error.contains("Invalid decimal string 'invalid'"));

        let error = f32::NAN.try_into_decimal().unwrap_err();
        assert!(error.contains("Invalid float value"));

        let error = f64::INFINITY.try_into_decimal().unwrap_err();
        assert!(error.contains("Invalid float value"));
    }
}
