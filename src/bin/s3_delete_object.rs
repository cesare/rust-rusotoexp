use anyhow::Result;
use rusoto_core::Region;
use rusoto_s3::{DeleteObjectRequest, S3Client, S3};
use structopt::StructOpt;
use tokio;

#[derive(StructOpt, Debug)]
#[structopt(name = "s3_delete_objects")]
struct Options {
    #[structopt(short, long)]
    bucket: String,

    #[structopt(short, long)]
    key: String,
}

impl Options {
    fn create_request(&self) -> DeleteObjectRequest {
        DeleteObjectRequest {
            bucket: self.bucket.to_owned(),
            bypass_governance_retention: None,
            expected_bucket_owner: None,
            key: self.key.to_owned(),
            mfa: None,
            request_payer: None,
            version_id: None,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let options = Options::from_args();

    let region = Region::default();
    let client = S3Client::new(region);
    let request = options.create_request();
    let response = client.delete_object(request).await?;
    print!("{:?}", response);
    Ok(())
}
