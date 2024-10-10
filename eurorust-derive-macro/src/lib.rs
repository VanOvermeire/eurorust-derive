use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Sender)]
pub fn something(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let original_struct_name = ast.ident;
    let new_struct_name = format_ident!("SqsClientFor{}", original_struct_name);

    quote!(
        struct #new_struct_name {
            client: aws_sdk_sqs::Client,
            queue_url: String
        }

        impl #new_struct_name {
            pub fn new(client: aws_sdk_sqs::Client, queue_url: String) -> Self {
                Self {
                    client,
                    queue_url,
                }
            }

            pub async fn send(&self, message: #original_struct_name) {
                self.client.send_message()
                    .queue_url(&self.queue_url)
                    .message_body(serde_json::to_string(&message).expect("conversion failed"))
                    .send()
                    .await
                    .expect("send message failed");
            }

            pub async fn receive(&self) -> Vec<#original_struct_name> {
                    let response = self.client.receive_message()
                        .queue_url(&self.queue_url)
                        .send()
                        .await
                        .expect("receive message failed");
                    response.messages
                        .expect("no messages in response")
                        .into_iter()
                        .map(|m| m.body.expect("no body in response message"))
                        .map(|m| serde_json::from_str(&m).expect("message not valid Message struct"))
                        .collect()
            }
        }
    ).into()
}