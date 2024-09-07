use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Sender)]
pub fn eurorust(args: TokenStream) -> TokenStream {
    let input = parse_macro_input!(args as DeriveInput);
    let name = input.ident;
    let struct_name = format_ident!("SqsClientFor{}", name);

    quote!(
        pub struct #struct_name {
            client: aws_sdk_sqs::Client,
            queue_url: String,
        }

        impl #struct_name {
            pub fn new(client: aws_sdk_sqs::Client, queue_url: String) -> Self {
                Self { client, queue_url }
            }

            pub async fn send(&self, message: #name) {
                self.client.send_message()
                    .queue_url(&self.queue_url)
                    .message_body(serde_json::to_string(&message).expect("conversion to json to work"))
                    .send()
                    .await
                    .expect("send message to queue to succeed");
            }

            pub async fn receive(&self) -> Vec<#name> {
                let response = self.client.receive_message()
                    .queue_url(&self.queue_url)
                    .send()
                    .await
                    .expect("receive message to queue to succeed");

                response.messages.expect("to have at least one message")
                    .into_iter()
                    .map(|m| m.body.expect("body to be part of message"))
                    .map(|m| serde_json::from_str(&m).expect("message to be valid Message struct"))
                    .collect::<Vec<#name>>()
            }
        }
    ).into()
}
