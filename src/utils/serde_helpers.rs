use rust_decimal::Decimal;
use serde::{de, Deserialize, Deserializer};

use crate::models::{CryptoCurrencyCode, CurrencyCode, FiatCurrencyCode};

pub fn serialize_comma_separated_list<S>(
    ids: &Option<Vec<u64>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    if let Some(ids) = ids {
        let str_value = ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");
        serializer.serialize_str(&str_value)
    } else {
        unreachable!("should be skipped by skip_serializing_if")
    }
}

/// Deserialize a number to a Decimal
pub fn deserialize_decimal_from_number<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    let num = f64::deserialize(deserializer)?;
    Decimal::try_from(num).map_err(D::Error::custom)
}

/// Deserialize a String to a Decimal
pub fn deserialize_decimal_from_string<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse().map_err(de::Error::custom)
}

/// Serialize a Decimal to a String
pub fn serialize_decimal_to_string<S>(value: &Decimal, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&value.to_string())
}

/// Deserialize an optional String to a Decimal
pub fn deserialize_optional_decimal_from_string<'de, D>(
    deserializer: D,
) -> Result<Option<Decimal>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Helper(#[serde(deserialize_with = "deserialize_decimal_from_string")] Decimal);

    let helper = Option::deserialize(deserializer)?;
    Ok(helper.map(|Helper(dec)| dec))
}

/// Deserialize a String to a CurrencyCode
pub fn deserialize_currency_code<'de, D>(deserializer: D) -> Result<CurrencyCode, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let code = String::deserialize(deserializer)?;

    if let Ok(crypto) = serde_json::from_str::<CryptoCurrencyCode>(&format!("\"{}\"", code)) {
        return Ok(CurrencyCode::Crypto(crypto));
    }

    if let Ok(fiat) = serde_json::from_str::<FiatCurrencyCode>(&format!("\"{}\"", code)) {
        return Ok(CurrencyCode::Fiat(fiat));
    }

    Err(serde::de::Error::custom(format!(
        "Invalid currency code: {}",
        code
    )))
}
