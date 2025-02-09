mod serde_helpers;

pub use serde_helpers::*;

#[cfg(test)]
pub mod test_utils {
    use mockito::ServerGuard;
    use tokio::runtime::Runtime;
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
    }
}
