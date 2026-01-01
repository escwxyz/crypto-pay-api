use crypto_pay_api::prelude::*;

#[test]
fn test_amount_with_dec_macro() {
    let client = CryptoBot::builder().api_token("token").build().unwrap();
    let _ = client
        .create_invoice()
        .asset(CryptoCurrencyCode::Ton)
        .amount(dec!(10.5)); // Works
}

#[test]
fn test_amount_with_literal() {
    let client = CryptoBot::builder().api_token("token").build().unwrap();
    let _ = client.create_invoice().asset(CryptoCurrencyCode::Ton).amount(10.5); // Should work
    let _ = client.create_invoice().asset(CryptoCurrencyCode::Ton).amount(10); // Should work
}
