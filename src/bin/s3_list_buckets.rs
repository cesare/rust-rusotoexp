use anyhow::Result;
use rusoto_core::Region;
use rusoto_s3::{S3Client, S3};
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let region = Region::default();
    let client = S3Client::new(region);
    let results = client.list_buckets().await?;
    print!("{:?}", results);
    Ok(())
}
