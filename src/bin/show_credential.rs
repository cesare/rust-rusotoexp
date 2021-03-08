use anyhow::Result;
use rusoto_credential::{AwsCredentials, ChainProvider, ProvideAwsCredentials};
use tokio;

async fn find_credentials() -> Result<AwsCredentials> {
    let provider = ChainProvider::new();
    let credentials = provider.credentials().await?;
    Ok(credentials)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credentials = find_credentials().await?;
    println!("Found AWS credentials: {:?}", credentials);
    Ok(())
}
