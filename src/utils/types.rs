pub use reqwest::header::{HeaderName, HeaderValue};
pub use reqwest::Client;
pub use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
pub use rust_decimal::Decimal;
pub use rust_decimal_macros::dec;
pub use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub trait IntoDecimal {
    fn into_decimal(self) -> Decimal;
}

impl IntoDecimal for Decimal {
    fn into_decimal(self) -> Decimal {
        self
    }
}

impl IntoDecimal for &Decimal {
    fn into_decimal(self) -> Decimal {
        *self
    }
}

macro_rules! impl_into_decimal_int {
    ($($t:ty),*) => {
        $(
            impl IntoDecimal for $t {
                fn into_decimal(self) -> Decimal {
                    Decimal::from(self)
                }
            }
        )*
    };
}

impl_into_decimal_int!(u8, i8, u16, i16, u32, i32, u64, i64, isize, usize);

impl IntoDecimal for f32 {
    fn into_decimal(self) -> Decimal {
        Decimal::from_f32(self).unwrap_or_else(|| panic!("Invalid float value: {}", self))
    }
}

impl IntoDecimal for f64 {
    fn into_decimal(self) -> Decimal {
        Decimal::from_f64(self).unwrap_or_else(|| panic!("Invalid float value: {}", self))
    }
}

impl IntoDecimal for &str {
    fn into_decimal(self) -> Decimal {
        Decimal::from_str(self).unwrap_or_else(|e| panic!("Invalid decimal string '{}': {}", self, e))
    }
}

impl IntoDecimal for String {
    fn into_decimal(self) -> Decimal {
        Decimal::from_str(&self).unwrap_or_else(|e| panic!("Invalid decimal string '{}': {}", self, e))
    }
}

impl IntoDecimal for &String {
    fn into_decimal(self) -> Decimal {
        Decimal::from_str(self).unwrap_or_else(|e| panic!("Invalid decimal string '{}': {}", self, e))
    }
}
