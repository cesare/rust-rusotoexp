use rusoto_credential::{ChainProvider, ProvideAwsCredentials};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = ChainProvider::new();
    let credentials = provider.credentials().await?;
    println!("Found AWS credentials: {:?}", credentials);
    Ok(())
}
