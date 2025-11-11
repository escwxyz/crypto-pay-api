use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use std::str::FromStr;

use crate::models::{CryptoCurrencyCode, CurrencyCode, FiatCurrencyCode};

/// Serialize a comma-separated list of u64 to a String
pub fn serialize_comma_separated_list<S>(ids: &Option<Vec<u64>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    if let Some(ids) = ids {
        let str_value = ids.iter().map(|id| id.to_string()).collect::<Vec<_>>().join(",");
        serializer.serialize_str(&str_value)
    } else {
        unreachable!("should be skipped by skip_serializing_if")
    }
}

/// Deserialize a Decimal from either a JSON number or a JSON string containing a number.
fn deserialize_decimal_from_number_or_string<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;

    let v = Value::deserialize(deserializer)?;

    match v {
        Value::Number(n) => {
            // Try to convert number to Decimal preserving integers precisely when possible.
            if let Some(i) = n.as_i64() {
                Ok(Decimal::from(i))
            } else if let Some(u) = n.as_u64() {
                Ok(Decimal::from(u))
            } else if let Some(f) = n.as_f64() {
                Decimal::try_from(f).map_err(D::Error::custom)
            } else {
                Err(D::Error::custom("invalid numeric value for Decimal"))
            }
        }
        Value::String(s) => Decimal::from_str(&s).map_err(D::Error::custom),
        other => Err(D::Error::custom(format!(
            "unexpected JSON value for Decimal: {:?}",
            other
        ))),
    }
}

/// Deserialize a number to a Decimal
pub fn deserialize_decimal_from_number<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    deserialize_decimal_from_number_or_string(deserializer)
}

/// Deserialize a String to a Decimal
pub fn deserialize_decimal_from_string<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    deserialize_decimal_from_number_or_string(deserializer)
}

/// Serialize a Decimal to a String
pub fn serialize_decimal_to_string<S>(value: &Decimal, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&value.to_string())
}

/// Deserialize an optional String to a Decimal
pub fn deserialize_optional_decimal_from_string<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
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

    if let Ok(crypto) = serde_json::from_str::<CryptoCurrencyCode>(&format!("\"{code}\"")) {
        if crypto != CryptoCurrencyCode::Unknown {
            return Ok(CurrencyCode::Crypto(crypto));
        }
    }

    if let Ok(fiat) = serde_json::from_str::<FiatCurrencyCode>(&format!("\"{code}\"")) {
        if fiat != FiatCurrencyCode::Unknown {
            return Ok(CurrencyCode::Fiat(fiat));
        }
    }

    Err(serde::de::Error::custom(format!("Invalid currency code: {code}")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    #[derive(Debug, Serialize)]
    struct TestCommaList {
        #[serde(
            skip_serializing_if = "Option::is_none",
            serialize_with = "serialize_comma_separated_list"
        )]
        ids: Option<Vec<u64>>,
    }

    #[derive(Debug, Deserialize)]
    struct TestDecimalNumber {
        #[serde(deserialize_with = "deserialize_decimal_from_number")]
        value: Decimal,
    }

    #[derive(Debug, Deserialize)]
    struct TestDecimalString {
        #[serde(deserialize_with = "deserialize_decimal_from_string")]
        value: Decimal,
    }

    #[derive(Debug, Serialize)]
    struct TestDecimalToString {
        #[serde(serialize_with = "serialize_decimal_to_string")]
        value: Decimal,
    }

    #[derive(Debug, Deserialize)]
    struct TestOptionalDecimal {
        #[serde(deserialize_with = "deserialize_optional_decimal_from_string")]
        value: Option<Decimal>,
    }

    #[test]
    fn test_serialize_comma_separated_list() {
        // Test with Some values
        let test = TestCommaList {
            ids: Some(vec![1, 2, 3]),
        };
        let serialized = serde_json::to_value(&test).unwrap();
        assert_eq!(serialized["ids"], "1,2,3");

        // Test with empty vec
        let test = TestCommaList { ids: Some(vec![]) };
        let serialized = serde_json::to_value(&test).unwrap();
        assert_eq!(serialized["ids"], "");

        // Test with None (should be skipped)
        let test = TestCommaList { ids: None };
        let serialized = serde_json::to_value(&test).unwrap();
        assert!(!serialized.as_object().unwrap().contains_key("ids"));
    }

    #[test]
    fn test_deserialize_decimal_from_number() {
        // Test integer
        let json = json!({"value": 42});
        let result: TestDecimalNumber = serde_json::from_value(json).unwrap();
        assert_eq!(result.value, dec!(42));

        // Test float
        let json = json!({"value": 42.5});
        let result: TestDecimalNumber = serde_json::from_value(json).unwrap();
        assert_eq!(result.value, dec!(42.5));

        // Test negative
        let json = json!({"value": -42.5});
        let result: TestDecimalNumber = serde_json::from_value(json).unwrap();
        assert_eq!(result.value, dec!(-42.5));
    }

    #[test]
    fn test_deserialize_decimal_from_string() {
        // Test integer string
        let json = json!({"value": "42"});
        let result: TestDecimalString = serde_json::from_value(json).unwrap();
        assert_eq!(result.value, dec!(42));

        // Test decimal string
        let json = json!({"value": "42.5"});
        let result: TestDecimalString = serde_json::from_value(json).unwrap();
        assert_eq!(result.value, dec!(42.5));

        // Test negative string
        let json = json!({"value": "-42.5"});
        let result: TestDecimalString = serde_json::from_value(json).unwrap();
        assert_eq!(result.value, dec!(-42.5));

        // Test invalid string
        let json = json!({"value": "invalid"});
        assert!(serde_json::from_value::<TestDecimalString>(json).is_err());
    }

    #[test]
    fn test_deserialize_decimal_from_number_errors() {
        // Test number that's too large for Decimal
        let json = json!({"value": 1e100});
        let result: Result<TestDecimalNumber, _> = serde_json::from_value(json);
        assert!(result.is_err());

        // Test NaN
        let json = json!({"value": f64::NAN});
        let result: Result<TestDecimalNumber, _> = serde_json::from_value(json);
        assert!(result.is_err());

        // Test Infinity
        let json = json!({"value": f64::INFINITY});
        let result: Result<TestDecimalNumber, _> = serde_json::from_value(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_decimal_from_string_errors() {
        // Test invalid decimal string
        let json = json!({"value": "not_a_number"});
        let result: Result<TestDecimalString, _> = serde_json::from_value(json);
        assert!(result.is_err());

        // Test empty string
        let json = json!({"value": ""});
        let result: Result<TestDecimalString, _> = serde_json::from_value(json);
        assert!(result.is_err());

        // Test malformed decimal
        let json = json!({"value": "123.456.789"});
        let result: Result<TestDecimalString, _> = serde_json::from_value(json);
        assert!(result.is_err());

        // Test number out of range
        let json = json!({"value": "1e100"});
        let result: Result<TestDecimalString, _> = serde_json::from_value(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_serialize_decimal_to_string() {
        let test = TestDecimalToString { value: dec!(42.5) };
        let serialized = serde_json::to_value(&test).unwrap();
        assert_eq!(serialized["value"], "42.5");

        let test = TestDecimalToString { value: dec!(-42.5) };
        let serialized = serde_json::to_value(&test).unwrap();
        assert_eq!(serialized["value"], "-42.5");
    }

    #[test]
    fn test_deserialize_optional_decimal_from_string() {
        // Test Some value
        let json = json!({"value": "42.5"});
        let result: TestOptionalDecimal = serde_json::from_value(json).unwrap();
        assert_eq!(result.value, Some(dec!(42.5)));

        // Test None
        let json = json!({"value": null});
        let result: TestOptionalDecimal = serde_json::from_value(json).unwrap();
        assert_eq!(result.value, None);

        // Test invalid string
        let json = json!({"value": "invalid"});
        assert!(serde_json::from_value::<TestOptionalDecimal>(json).is_err());
    }

    #[test]
    fn test_deserialize_currency_code() {
        // Test valid crypto currency
        let result = deserialize_currency_code(&mut serde_json::de::Deserializer::from_str("\"BTC\"")).unwrap();
        assert!(matches!(result, CurrencyCode::Crypto(CryptoCurrencyCode::Btc)));

        // Test valid fiat currency
        let result = deserialize_currency_code(&mut serde_json::de::Deserializer::from_str("\"USD\"")).unwrap();
        assert!(matches!(result, CurrencyCode::Fiat(FiatCurrencyCode::Usd)));

        // Test invalid currency
        let result = deserialize_currency_code(&mut serde_json::de::Deserializer::from_str("\"XXX\""));
        assert!(result.is_err());
    }
}
