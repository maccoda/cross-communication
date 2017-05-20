extern crate grpc;
extern crate chat_client;

use chat_client::message::{self, InitiateRequest, TerminateRequest, MessageRequest};
use chat_client::message_grpc::*;

const ROOM: &str = "cross_comm";
fn main() {
    // Make the client from the generated code
    let client = CommunicatorClient::new("localhost", 50051, false, Default::default()).unwrap();

    // Initiate
    let mut req = InitiateRequest::new();
    req.set_clientAddress(make_address());
    req.set_room(make_room());
    let response = client.initiate_conversation(::grpc::RequestOptions::new(), req);
    println!("Initiate Response: {:?}", response.wait_drop_metadata());

    // Message
    let mut req = MessageRequest::new();
    req.set_clientAddress(make_address());
    req.set_room(make_room());
    req.set_message("Hello there".to_owned());

    let response = client.send_message(::grpc::RequestOptions::new(),
                                       ::grpc::StreamingRequest::once(req));
    println!("Message Response: {:?}",
             response.single().wait_drop_metadata());

    // Terminate
    let mut req = TerminateRequest::new();
    req.set_clientAddress(make_address());
    req.set_room(make_room());

    let response = client.terminate_conversation(::grpc::RequestOptions::new(), req);
    println!("Terminate Response: {:?}", response.wait_drop_metadata());

}

fn make_address() -> message::Address {
    let mut addr = message::Address::new();
    addr.set_address("local".to_owned());
    addr
}

fn make_room() -> message::Room {
    let mut room = message::Room::new();
    room.set_name(ROOM.to_owned());
    room
}
