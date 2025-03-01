mod serde_helpers;
pub mod types;

pub use serde_helpers::*;

#[cfg(test)]
pub mod test_utils {
    use mockito::ServerGuard;
    use rust_decimal_macros::dec;
    use tokio::runtime::Runtime;

    use crate::models::{CryptoCurrencyCode, ExchangeRate, FiatCurrencyCode};
    pub struct TestContext {
        pub server: ServerGuard,
        pub runtime: Runtime,
    }

    impl TestContext {
        pub fn new() -> Self {
            Self {
                server: mockito::Server::new(),
                runtime: Runtime::new().expect("Failed to create runtime"),
            }
        }

        pub fn run<F, T>(&self, future: F) -> T
        where
            F: std::future::Future<Output = T>,
        {
            println!("Starting to run test");
            let result = self.runtime.block_on(future);
            println!("Test completed");
            result
        }

        pub fn mock_exchange_rates() -> Vec<ExchangeRate> {
            vec![
                ExchangeRate {
                    is_valid: true,
                    is_crypto: true,
                    is_fiat: false,
                    source: CryptoCurrencyCode::Ton,
                    target: FiatCurrencyCode::Usd,
                    rate: dec!(3.70824926),
                },
                ExchangeRate {
                    is_valid: true,
                    is_crypto: true,
                    is_fiat: false,
                    source: CryptoCurrencyCode::Ton,
                    target: FiatCurrencyCode::Eur,
                    rate: dec!(3.59048268),
                },
            ]
        }
    }
}
