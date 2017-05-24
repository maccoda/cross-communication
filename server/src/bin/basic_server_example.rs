extern crate chat_server;
extern crate grpc;
extern crate futures;
extern crate futures_cpupool;
extern crate protobuf;

use std::thread;

use futures_cpupool::CpuPool;

use grpc::{SingleResponse, StreamingResponse, StreamingRequest};

use chat_server::message_grpc::*;
use chat_server::message::*;

/// Type for the *rooms* that the server uses.
#[derive(PartialEq)]
struct MsgRoom(Room);

impl MsgRoom {
    fn name(&self) -> &str {
        self.0.get_name()
    }

    fn from(name: &str) -> MsgRoom {
        let mut room = Room::new();
        room.set_name(name.to_owned());
        MsgRoom(room)
    }
}


struct CommunicatorImpl {
    // NOTE This is just the basic implementation, a map or some sorted structure may be better
    // NOTE Would also need to have access to the database of conversation history
    conversations: Vec<MsgRoom>,
}

impl CommunicatorImpl {
    fn new() -> CommunicatorImpl {
        CommunicatorImpl { conversations: vec![MsgRoom::from("cross_comm")] }
    }
}

impl Communicator for CommunicatorImpl {
    fn initiate_conversation(&self,
                             options: ::grpc::RequestOptions,
                             req: InitiateRequest)
                             -> SingleResponse<InitiateReply> {
        let mut reply = InitiateReply::new();
        println!("Received an initiate command from {:?}",
                 req.get_clientAddress());
        println!("They wish to connect with {:?}", req.get_room());
        let matches: Vec<&MsgRoom> = self.conversations
            .iter()
            .filter(|x| x.name() == req.get_room().get_name())
            .collect();
        if matches.len() != 1 {
            SingleResponse::err(grpc::error::Error::Other("Room unavailable to open"))
        } else {
            reply.set_success(true);
            SingleResponse::completed(reply)
        }
    }

    fn terminate_conversation(&self,
                              options: ::grpc::RequestOptions,
                              req: TerminateRequest)
                              -> SingleResponse<TerminateReply> {
        let mut reply = TerminateReply::new();
        println!("Received a terminate command from {:?}",
                 req.get_clientAddress());
        let req_id = req.get_room();
        println!("They wish to end their conversation with {:?}", req_id);
        // First check that the conversation can be ended
        let matches: Vec<&MsgRoom> = self.conversations
            .iter()
            .filter(|x| x.name() == req.get_room().get_name())
            .collect();
        if matches.len() != 1 {
            SingleResponse::err(grpc::error::Error::Other("Conversation not yet open. Incorrect \
                                                       conversation ID"))
        } else {
            // matches[0].clear();
            reply.set_success(true);
            SingleResponse::completed(reply)
        }

    }
    fn send_message(&self,
                    options: ::grpc::RequestOptions,
                    reqs: StreamingRequest<MessageRequest>)
                    -> StreamingResponse<MessageReply> {
        // FIXME Unsure of how to make mock of the iterator
        let mut msg = Message::new();
        msg.set_content("Message here".to_string());
        msg.set_user(make_address());
        let mut reply = MessageReply::new();
        reply.set_messages(msg);
        StreamingResponse::completed(vec![reply])
    }
}
fn make_address() -> Address {
    let mut addr = Address::new();
    addr.set_address("remote".to_owned());
    addr
}

#[allow(unused_variables)]
fn main() {
    // Create the server, need unused variable so doesn't get disposed of
    let server = CommunicatorServer::new_pool("[::]:50051",
                                              Default::default(),
                                              CommunicatorImpl::new(),
                                              CpuPool::new(4));
    println!("Server started");
    loop {
        thread::park();
    }
}
