use aws_config::BehaviorVersion;
use aws_sdk_sqs::{Client, Error};
use serde::Deserialize;

const REGION: &str = "eu-west-1";
const ENDPOINT: &str = "http://localhost:4566";

#[derive(Debug,Deserialize)]
struct Message {
    name: String,
    country: String,
}

// TODO now add a proc macro that
// - generates a client
// - with a new method that takes the queue
// - and a send and receive that automatically change it into the proper struct

#[tokio::main]
async fn main() -> Result<(), aws_sdk_sqs::Error> {
    basic_example().await?;
    Ok(())
}

async fn basic_example() -> Result<(), Error> {
    let sqs_client = create_sqs_client().await;
    let our_queue = find_queue(&sqs_client).await?;

    sqs_client.send_message()
        .queue_url(our_queue)
        .message_body(r#"{ "name": "John", "country": "UK" }"#)
        .send()
        .await
        .expect("send message to queue to succeed");

    let response = sqs_client.receive_message()
        .queue_url(our_queue)
        .send()
        .await
        .expect("receive message to queue to succeed");

    let messages = response.messages.expect("to have at least one message")
        .into_iter()
        .map(|m| m.body.expect("body to be part of message"))
        .map(|m| serde_json::from_str(&m).expect("message to be valid Message struct"))
        .collect::<Vec<Message>>();

    println!("Received {:?} from {}", messages, our_queue);
    Ok(())
}

async fn find_queue(sqs_client: &Client) -> Result<&String, Error> {
    let queues = sqs_client.list_queues().send().await?;
    let queue_urls = queues.queue_urls.expect("to have at least one queue");
    let our_queue = queue_urls.first().expect("to have at least one queue");
    Ok(our_queue)
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
