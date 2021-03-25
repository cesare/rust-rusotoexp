use anyhow::Result;
use rusoto_core::Region;
use rusoto_s3::{ListObjectsV2Request, Object, S3Client, S3};
use structopt::StructOpt;
use tokio;

#[derive(StructOpt, Debug)]
#[structopt(name = "s3_list_objects")]
struct Options {
    #[structopt(short, long)]
    bucket: String,
}

impl Options {
    fn create_initial_request(&self) -> ListObjectsV2Request {
        ListObjectsV2Request {
            bucket: self.bucket.to_owned(),
            continuation_token: None,
            delimiter: None,
            encoding_type: None,
            expected_bucket_owner: None,
            fetch_owner: Some(true),
            max_keys: None,
            prefix: None,
            request_payer: None,
            start_after: None,
        }
    }
}

fn show_object(object: &Object) {
    if let Some(key) = &object.key {
        if let Some(Ok(last_modified)) = object
            .last_modified
            .as_ref()
            .map(|dt| chrono::DateTime::parse_from_rfc3339(dt))
        {
            println!("{} {}", last_modified, key);
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let options = Options::from_args();

    let region = Region::default();
    let client = S3Client::new(region);
    let request = options.create_initial_request();
    let results = client.list_objects_v2(request).await?;

    if let Some(contents) = results.contents {
        for object in contents.iter() {
            show_object(&object);
        }
    }

    Ok(())
}
