use anyhow::Result;
use rusoto_core::Region;
use rusoto_ssm::{GetParameterRequest, Ssm, SsmClient};
use structopt::StructOpt;
use tokio;

#[derive(StructOpt, Debug)]
#[structopt(name = "ssm_get_parameter")]
struct Options {
    #[structopt(short, long)]
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = Options::from_args();

    let region = Region::default();
    let client = SsmClient::new(region);
    let request = GetParameterRequest {
        name: options.name.to_owned(),
        with_decryption: None,
    };

    let result = client.get_parameter(request).await?;
    println!("{:?}", result);

    Ok(())
}
