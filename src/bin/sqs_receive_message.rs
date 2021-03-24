use anyhow::Result;
use rusoto_core::Region;
use rusoto_sqs::{DeleteMessageRequest, ReceiveMessageRequest, Sqs, SqsClient};
use structopt::StructOpt;
use tokio;

#[derive(StructOpt, Debug)]
#[structopt(name = "sqs_receive_message")]
struct Options {
    #[structopt(short, long)]
    queue_url: String,
}

impl Options {
    fn create_sqs_client(&self) -> SqsClient {
        let region = self.region();
        SqsClient::new(region)
    }

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

    fn region(&self) -> Region {
        Region::default()
    }
}

async fn delete_message(options: &Options, receipt_handle: &str) -> Result<()> {
    let client = options.create_sqs_client();
    let request = DeleteMessageRequest {
        queue_url: options.queue_url.to_owned(),
        receipt_handle: receipt_handle.to_owned(),
    };
    client.delete_message(request).await?;
    Ok(())
}

async fn wait_for_messages(options: &Options) -> Result<()> {
    let client = options.create_sqs_client();
    let request = options.create_request();
    let results = client.receive_message(request).await?;

    if let Some(messages) = results.messages {
        for message in messages {
            println!("{:?}", message);

            if let Some(receipt_handle) = message.receipt_handle {
                delete_message(&options, &receipt_handle).await?;
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let options = Options::from_args();
    loop {
        wait_for_messages(&options).await?;
    }
}
