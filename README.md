# Crypto Pay API Client for Rust

[![Crates.io](https://img.shields.io/crates/v/crypto-pay-api.svg)](https://crates.io/crates/crypto-pay-api)
[![Documentation](https://docs.rs/crypto-pay-api/badge.svg)](https://docs.rs/crypto-pay-api)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![CI](https://github.com/escwxyz/crypto-pay-api/actions/workflows/test.yml/badge.svg)](https://github.com/escwxyz/crypto-pay-api/actions/workflows/test.yml)
[![codecov](https://codecov.io/gh/escwxyz/crypto-pay-api/graph/badge.svg?token=Ar6BnDUude)](https://codecov.io/gh/escwxyz/crypto-pay-api)

A type-safe Rust client for the [Crypto Bot](https://t.me/CryptoBot) API with async support.

## Features

- Complete type safety with typestate builder pattern
- Async/await support
- Comprehensive error handling
- Built-in parameter validation
- Zero configuration
- Webhook support
- Full API coverage

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
crypto-pay-api = "0.2.0"
```

### Basic Example

```rust
use crypto_pay_api::prelude::*;

#[tokio::main]
async fn main() -> Result<(), CryptoBotError> {
    // Initialize client
    let client = CryptoBot::builder()
        .api_token("YOUR_API_TOKEN")
        .build()?;

    // Create an invoice using the builder pattern
    let invoice = client.create_invoice()
        .asset(CryptoCurrencyCode::Ton)
        .amount(10.5) // Accepts numeric literals directly
        .description("Test payment".to_string())
        .execute()
        .await?;
    
    println!("Payment URL: {}", invoice.bot_invoice_url);

    Ok(())
}
```

## API Usage

All API methods follow a consistent builder pattern:

```rust
client.api_method()
    .optional_param(value)
    .execute()
    .await?
```

### Creating Invoices

```rust
let invoice = client.create_invoice()
    .asset(CryptoCurrencyCode::Ton)
    .amount(100.0) // No need for dec!() macro
    .description("Premium subscription".to_string())
    .payload("user_123".to_string())
    .execute()
    .await?;
```

### Querying Invoices

```rust
let invoices = client.get_invoices()
    .asset(CryptoCurrencyCode::Ton)
    .invoice_ids(vec![123, 456])
    .count(50)
    .execute()
    .await?;
```

### Deleting Invoices

```rust
let success = client.delete_invoice(invoice_id)
    .execute()
    .await?;
```

### Creating Transfers

```rust
let transfer = client.transfer()
    .user_id(123456789)
    .asset(CryptoCurrencyCode::Usdt)
    .amount(50.0) // Flexible amount input
    .spend_id("unique_id_123".to_string())
    .comment("Payment for services".to_string())
    .execute()
    .await?;
```

### Getting Balance

```rust
let balances = client.get_balance()
    .execute()
    .await?;

for balance in balances {
    println!("{}: {}", balance.currency_code, balance.available);
}
```

### Getting Exchange Rates

```rust
let rates = client.get_exchange_rates()
    .execute()
    .await?;
```

### Getting Statistics

```rust
let stats = client.get_stats()
    .start_at(Utc::now() - Duration::days(7))
    .end_at(Utc::now())
    .execute()
    .await?;
```

## API Coverage

### Invoices

- Create invoice (`create_invoice`)
- Get invoices (`get_invoices`)
- Delete invoice (`delete_invoice`)

### Transfers

- Transfer (`transfer`)
- Get transfers (`get_transfers`)

### Checks

- Create check (`create_check`)
- Get checks (`get_checks`)
- Delete check (`delete_check`)

### Other Features

- Get balance (`get_balance`)
- Get exchange rates (`get_exchange_rates`)
- Get currencies (`get_currencies`)
- Get app info (`get_me`)
- Get statistics (`get_stats`)

## Webhook Handling

```rust
use crypto_pay_api::prelude::*;

#[tokio::main]
async fn main() -> Result<(), CryptoBotError> {
    let client = CryptoBot::builder()
        .api_token("YOUR_API_TOKEN")
        .build()?;
    
    let mut handler = client.webhook_handler().build();

    // Register payment callback
    handler.on_update(|update| async move {
        println!("Invoice paid: {:?}", update.payload);
        Ok(())
    });

    // Start webhook server
    // ... integrate with your web framework
    Ok(())
}
```

See [examples/axum_webhook.rs](examples/axum_webhook.rs) for a complete example using axum.

## Custom Configuration

```rust
let client = CryptoBot::builder()
    .api_token("YOUR_API_TOKEN")
    .base_url("https://pay.crypt.bot/api")
    .timeout(Duration::from_secs(30))
    .build()?;
```

## Error Handling

The library provides detailed error types:

```rust
match client.get_balance().execute().await {
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

## Documentation

- [API Documentation](https://docs.rs/crypto-pay-api)
- [Crypto Bot API Documentation](https://help.crypt.bot/crypto-pay-api)

## Contributing

Contributions are welcome! Please check out our [Contributing Guide](CONTRIBUTING.md).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
