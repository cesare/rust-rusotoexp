use anyhow::Result;
use futures::stream::StreamExt;
use rusoto_core::{ByteStream, Region};
use rusoto_s3::{GetObjectRequest, S3Client, S3};
use structopt::StructOpt;
use tokio;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use std::path::PathBuf;

#[derive(StructOpt, Debug)]
#[structopt(name = "s3_get_object")]
struct Options {
    #[structopt(short, long)]
    bucket: String,

    #[structopt(short, long)]
    key: String,

    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

impl Options {
    fn create_request(&self) -> GetObjectRequest {
        GetObjectRequest {
            bucket: self.bucket.to_owned(),
            key: self.key.to_owned(),
            expected_bucket_owner: None,
            if_match: None,
            if_modified_since: None,
            if_none_match: None,
            if_unmodified_since: None,
            part_number: None,
            range: None,
            request_payer: None,
            response_cache_control: None,
            response_content_disposition: None,
            response_content_encoding: None,
            response_content_language: None,
            response_content_type: None,
            response_expires: None,
            sse_customer_algorithm: None,
            sse_customer_key: None,
            sse_customer_key_md5: None,
            version_id: None,
        }
    }
}

async fn download(path: &PathBuf, body: &mut ByteStream) -> Result<()> {
    let mut file = File::create(path).await?;
    while let Some(Ok(bytes)) = body.next().await {
        file.write_all(bytes.as_ref()).await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let options = Options::from_args();

    let region = Region::default();
    let client = S3Client::new(region);
    let request = options.create_request();
    let response = client.get_object(request).await?;
    if let Some(mut body) = response.body {
        download(&options.path, &mut body).await?;
    }
    Ok(())
}
