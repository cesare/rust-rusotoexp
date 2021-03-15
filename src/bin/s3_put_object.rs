use anyhow::Result;
use rusoto_core::{ByteStream, Region};
use rusoto_s3::{PutObjectRequest, S3Client, S3};
use structopt::StructOpt;
use tokio;
use tokio::fs::File;
use tokio_util::io::ReaderStream;

use std::fs::Metadata;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
#[structopt(name = "s3_put_objects")]
struct Options {
    #[structopt(short, long)]
    bucket: String,

    #[structopt(short, long)]
    key: String,

    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

impl Options {
    fn create_request(&self, file: File, metadata: Metadata) -> PutObjectRequest {
        let stream = ReaderStream::new(file);
        let bytestream = ByteStream::new(stream);

        PutObjectRequest {
            acl: None,
            body: Some(bytestream),
            bucket: self.bucket.to_owned(),
            bucket_key_enabled: Some(false),
            cache_control: None,
            content_disposition: None,
            content_encoding: None,
            content_language: None,
            content_length: Some(metadata.len() as i64),
            content_md5: None,
            content_type: None,
            expected_bucket_owner: None,
            expires: None,
            grant_full_control: None,
            grant_read: None,
            grant_read_acp: None,
            grant_write_acp: None,
            key: self.key.to_owned(),
            metadata: None,
            object_lock_legal_hold_status: None,
            object_lock_mode: None,
            object_lock_retain_until_date: None,
            request_payer: None,
            sse_customer_algorithm: None,
            sse_customer_key: None,
            sse_customer_key_md5: None,
            ssekms_encryption_context: None,
            ssekms_key_id: None,
            server_side_encryption: None,
            storage_class: None,
            tagging: None,
            website_redirect_location: None,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let options = Options::from_args();

    let file = File::open(&options.path).await?;
    let metadata = file.metadata().await?;

    let region = Region::default();
    let client = S3Client::new(region);
    let request = options.create_request(file, metadata);
    let response = client.put_object(request).await?;
    println!("{:?}", response);
    Ok(())
}
