# Crypto Pay API Client for Rust 🦀

[![Crates.io](https://img.shields.io/crates/v/crypto-pay-api.svg)](https://crates.io/crates/crypto-pay-api)
[![Documentation](https://docs.rs/crypto-pay-api/badge.svg)](https://docs.rs/crypto-pay-api)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![CI](https://github.com/escwxyz/crypto-pay-api/actions/workflows/test.yml/badge.svg)](https://github.com/escwxyz/crypto-pay-api/actions/workflows/test.yml)
[![codecov](https://codecov.io/gh/escwxyz/crypto-pay-api/graph/badge.svg?token=Ar6BnDUude)](https://codecov.io/gh/escwxyz/crypto-pay-api)

A type-safe Rust client for the [Crypto Bot](https://t.me/CryptoBot) API with async support.

## Features ✨

- 🔒 Complete type safety
- 🚀 Async support
- 💡 Comprehensive error handling
- 🛠 Built-in parameter validation
- 📦 Zero configuration
- 🔌 Webhook support
- 📚 Full API coverage
- 🧪 Complete test coverage

## Quick Start 🚀

Add to your `Cargo.toml`:

```toml
[dependencies]
crypto-pay-api = "0.1.3"
```

### Basic Example with tokio

```rust
use crypto_pay_api::prelude::*;

#[tokio::main]
async fn main() -> Result<(), CryptoBotError> {
    // Initialize client
    let client = CryptoBot::builder().api_token("YOUR_API_TOKEN").build()?;

    // Create an invoice
    let params = CreateInvoiceParamsBuilder::new()
        .asset(CryptoCurrencyCode::Ton)
        .amount(dec!(10.5))
        .description("Test payment".to_string())
        .build(&client)
        .await?;

    let invoice = client.create_invoice(&params).await?;
    println!("Payment URL: {}", invoice.pay_url);

    Ok(())
}
```

## API Coverage 📋

### Invoices

- ✅ Create invoice (`create_invoice`)
- ✅ Get invoices (`get_invoices`)
- ✅ Delete invoice (`delete_invoice`)

### Transfers

- ✅ Transfer (`transfer`)
- ✅ Get transfers (`get_transfers`)

### Checks

- ✅ Create check (`create_check`)
- ✅ Get checks (`get_checks`)
- ✅ Delete check (`delete_check`)

### Other Features

- ✅ Get balance (`get_balance`)
- ✅ Get exchange rates (`get_exchange_rates`)
- ✅ Get currencies (`get_currencies`)
- ✅ Get app info (`get_me`)
- ✅ Get statistics (`get_stats`)

## Advanced Usage 🔧

### Webhook Handling

```rust
use crypto_pay_api::prelude::*;

#[tokio::main]
async fn main() -> Result<(), CryptoBotError> {
    let client = CryptoBot::builder().api_token("YOUR_API_TOKEN").build()?;
    let mut handler = client.webhook_handler(WebhookHandlerConfigBuilder::new().build());

    // Register payment callback
    handler.on_update(|update| async move {
        println!("Invoice paid: {:?}", update.payload);
        Ok(())
    });

    // Start webhook server
    // ... integrate with your web framework
}
```

See [examples/axum_webhook.rs](examples/axum_webhook.rs) for an example using axum.

### Custom Configuration

```rust
let client = CryptoBot::builder()
    .api_token("YOUR_API_TOKEN")
    .base_url("https://pay.crypt.bot/api")
    .timeout(Duration::from_secs(30))
    .build();
```

## Error Handling ⚠️

The library provides detailed error types:

```rust
match client.get_balance().await {
    Ok(balances) => {
        for balance in balances {
            println!("{}: {}", balance.currency_code, balance.available);
        }
    }
    Err(CryptoBotError::ValidationError { kind, message, field }) => {
        eprintln!("Validation error: {} (field: {:?})", message, field);
    }
    Err(e) => eprintln!("Other error: {}", e),
}
```

## Documentation 📚

- [API Documentation](https://docs.rs/crypto-pay-api)
- [Crypto Bot API Documentation](https://help.crypt.bot/crypto-pay-api)

## TODOs 🧪

- Add more integration tests

## Contributing 🤝

Contributions are welcome! Please check out our [Contributing Guide](CONTRIBUTING.md).

## License 📄

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
