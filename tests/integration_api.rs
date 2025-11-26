use crypto_pay_api::prelude::*;
use rust_decimal_macros::dec;

const API_TOKEN: &str = "28692:AAiEr9q60TVLrV0nGsl6KFaHJZnKgQDBcNU";
const BASE_URL: &str = "https://testnet-pay.crypt.bot/api";

async fn get_client() -> CryptoBot {
    CryptoBot::builder()
        .api_token(API_TOKEN)
        .base_url(BASE_URL)
        .build()
        .expect("Failed to create client")
}

#[tokio::test]
async fn test_get_me_real_api() {
    let client = get_client().await;

    let response = client.get_me().execute().await.expect("Failed to get me");

    println!("{:#?}", response);

    assert_eq!(response.name, "Stated Seaslug App");
    assert_eq!(response.app_id, 28692);
    assert_eq!(response.payment_processing_bot_username, "CryptoTestnetBot");
}

#[tokio::test]
async fn test_get_currencies_real_api() {
    let client = get_client().await;

    let response = client
        .get_currencies()
        .execute()
        .await
        .expect("Failed to get currencies");

    println!("{:#?}", response);

    assert!(!response.is_empty());
    let tether = response
        .iter()
        .find(|c| c.code == CurrencyCode::Crypto(CryptoCurrencyCode::Usdt))
        .expect("USDT not found in currencies");
    assert_eq!(tether.name, "Tether");
}

#[tokio::test]
async fn test_get_stats_real_api() {
    let client = get_client().await;

    let response = client.get_stats().execute().await.expect("Failed to get stats");

    println!("{:#?}", response);

    assert_eq!(response.volume, dec!(0));
    assert_eq!(response.conversion, dec!(0));
}

#[tokio::test]
async fn test_get_exchange_rates_real_api() {
    let client = get_client().await;

    let response = client
        .get_exchange_rates()
        .execute()
        .await
        .expect("Failed to get exchange rates");

    println!("{:#?}", response);

    assert!(!response.is_empty());

    let usdt_rub = response
        .iter()
        .find(|r| r.source == CryptoCurrencyCode::Usdt && r.target == FiatCurrencyCode::Rub)
        .expect("USDT→RUB rate not found");
    assert!(usdt_rub.rate > dec!(0));
}

#[tokio::test]
async fn test_invoice_real_api() {
    let client = get_client().await;

    let response = client
        .create_invoice()
        .amount(dec!(125.5))
        .asset(CryptoCurrencyCode::Ton)
        .execute()
        .await
        .expect("Failed to create invoice");

    println!("{:#?}", response);

    assert_eq!(response.amount, dec!(125.5));
    assert_eq!(response.asset, Some(CryptoCurrencyCode::Ton));

    let invoice_id = response.invoice_id;

    let invoices = client
        .get_invoices()
        .invoice_ids(vec![invoice_id])
        .execute()
        .await
        .expect("Failed to get invoices");

    println!("{:#?}", invoices);

    assert_eq!(invoices.len(), 1);
    assert_eq!(invoices[0].invoice_id, invoice_id);
    assert_eq!(invoices[0].amount, dec!(125.5));
    assert_eq!(invoices[0].asset, Some(CryptoCurrencyCode::Ton));

    let delete_response = client
        .delete_invoice(invoice_id)
        .execute()
        .await
        .expect("Failed to delete invoice");

    println!("{:#?}", delete_response);

    assert!(delete_response);
}

// TODO: add more tests for other APIs
