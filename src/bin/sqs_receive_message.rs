use anyhow::Result;
use rusoto_core::Region;
use rusoto_sqs::{ReceiveMessageRequest, Sqs, SqsClient};
use structopt::StructOpt;
use tokio;

#[derive(StructOpt, Debug)]
#[structopt(name = "sqs_receive_message")]
struct Options {
    #[structopt(short, long)]
    queue_url: String,
}

impl Options {
    fn create_request(&self) -> ReceiveMessageRequest {
        ReceiveMessageRequest {
            attribute_names: None,
            max_number_of_messages: Some(10),
            message_attribute_names: None,
            queue_url: self.queue_url.to_owned(),
            receive_request_attempt_id: None,
            visibility_timeout: Some(30),
            wait_time_seconds: Some(20),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let options = Options::from_args();
    println!("{:?}", options);
    let region = Region::default();
    let client = SqsClient::new(region);
    let request = options.create_request();
    let results = client.receive_message(request).await?;
    println!("{:?}", results);
    Ok(())
}
