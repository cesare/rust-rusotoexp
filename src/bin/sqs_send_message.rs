use anyhow::Result;
use rusoto_core::Region;
use rusoto_sqs::{SendMessageRequest, Sqs, SqsClient};
use structopt::StructOpt;
use tokio;

#[derive(StructOpt, Debug)]
#[structopt(name = "sqs_send_message")]
struct Options {
    #[structopt(short, long)]
    queue_url: String,

    #[structopt(short, long)]
    message: String,
}

impl Options {
    fn create_request(&self) -> SendMessageRequest {
        SendMessageRequest {
            delay_seconds: None,
            message_attributes: None,
            message_body: self.message.to_owned(),
            message_deduplication_id: None,
            message_group_id: None,
            message_system_attributes: None,
            queue_url: self.queue_url.to_owned(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let options = Options::from_args();

    let region = Region::default();
    let client = SqsClient::new(region);
    let request = options.create_request();
    let results = client.send_message(request).await?;
    println!("{:?}", results);
    Ok(())
}
