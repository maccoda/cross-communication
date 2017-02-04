extern crate grpc;
extern crate chat_client;

use chat_client::message::*;
use chat_client::message_grpc::*;


fn main() {
    println!("Hello, world!");

    // Make the client from the generated code
    let client = CommunicatorClient::new("localhost", 50051, false).unwrap();

    let mut req = ConversationControlRequest::new();
    req.set_receipient("My Friend".to_owned());
    let mut addr = Address::new();
    addr.set_address("local".to_owned());
    req.set_address(addr);
    let response = client.InitiateConversation(req);
    println!("Response: {:?}", response);
}
