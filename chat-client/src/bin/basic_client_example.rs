extern crate grpc;
extern crate chat_client;

use chat_client::message::{self, InitiateRequest};
use chat_client::message_grpc::*;


fn main() {
    // Make the client from the generated code
    let client = CommunicatorClient::new("localhost", 50051, false).unwrap();

    let mut req = InitiateRequest::new();
    req.set_receipient(message::User::new());
    let mut addr = message::Address::new();
    addr.set_address("local".to_owned());
    req.set_address(addr);
    let response = client.InitiateConversation(req);
    println!("Response: {:?}", response);
}
