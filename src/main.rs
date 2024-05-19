#![allow(dead_code, unused)]

use aws_config::BehaviorVersion;
use aws_sdk_sqs::{Client, Error};
use serde::{Deserialize, Serialize};

use eurorust_derive_macro::Sender;

const REGION: &str = "eu-west-1";
const ENDPOINT: &str = "http://localhost:4566";
const QUEUE_URL: &str = "http://sqs.eu-west-1.localhost.localstack.cloud:4566/000000000000/eurorust";

#[derive(Debug, Serialize, Deserialize, Sender)]
struct Message {
    name: String,
    country: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // basic_example().await?;
    derive_example().await?;
    Ok(())
}

async fn derive_example() -> Result<(), Error> {
    let sqs_client = create_sqs_client().await;

    let sqs_client = SqsClientForMessage::new(sqs_client, QUEUE_URL.to_string());
    sqs_client.send(Message {
        name: "Sam".to_string(),
        country: "Belgium".to_string(),
    }).await;
    let messages = sqs_client.receive().await;
    println!("Received {:?} from {}", messages, QUEUE_URL);

    Ok(())
}

async fn basic_example() -> Result<(), Error> {
    let sqs_client = create_sqs_client().await;

    let message_to_send = Message {
        name: "Sam".to_string(),
        country: "Belgium".to_string(),
    };

    sqs_client.send_message()
        .queue_url(QUEUE_URL)
        .message_body(serde_json::to_string(&message_to_send).expect("conversion to json to work"))
        .send()
        .await
        .expect("send message to queue to succeed");

    let response = sqs_client.receive_message()
        .queue_url(QUEUE_URL)
        .send()
        .await
        .expect("receive message to queue to succeed");

    let messages = response.messages.expect("to have at least one message")
        .into_iter()
        .map(|m| m.body.expect("body to be part of message"))
        .map(|m| serde_json::from_str(&m).expect("message to be valid Message struct"))
        .collect::<Vec<Message>>();

    println!("Received {:?} from {}", messages, QUEUE_URL);
    Ok(())
}

async fn create_sqs_client() -> Client {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(REGION)
        .endpoint_url(ENDPOINT)
        .load()
        .await;
    let sqs_client = aws_sdk_sqs::Client::new(&config);
    sqs_client
}
