//! This crate is for the communication server. The set up is such that there is
//! a single server to multiple client architecture. This server has a single
//! entry point through [`Server`], which will allow you construct and start the
//! server. Of course the server needs to be up prior to any clients being able
//! to connect.
//!
//! [`Server`]: struct.Server.html
#![warn(missing_docs)]
extern crate protobuf;
extern crate grpc;
extern crate futures;
extern crate futures_cpupool;


use std::collections::VecDeque;
use std::ops::DerefMut;

use grpc::{SingleResponse, StreamingResponse, StreamingRequest};
use grpc::futures_grpc::{self, GrpcStreamSend};
use futures_cpupool::CpuPool;
use message::*;
use message_grpc::{Communicator, CommunicatorServer};

pub mod message;
pub mod message_grpc;
#[cfg(test)]
mod test_utils;

/// grpc Server. This type represents the high level server type for receiving,
/// handling and redistributing all messages.
#[derive(Default)]
pub struct Server {
    conversations: Vec<MsgRoom>,
}

impl Server {
    /// Returns a new server with the configured allowed rooms set.
    pub fn new() -> Server {
        Server { conversations: vec![MsgRoom::from("cross_comm")] }
    }

    /// Consumes `self` to begin the grpc server.
    ///
    /// Ensure when using that keep a thread running and have the result bound so it is not dropped immediately.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::thread;
    ///
    /// #[allow(unused_variables)]
    /// fn main() {
    ///    // Create the server, need unused variable so doesn't get disposed of
    ///    let server = chat_server::Server::new().start();
    ///    println!("Server started");
    ///    // Start the thread (commented so tests halt)
    ///    //loop {
    ///    //    thread::park();
    ///    //}
    /// }
    /// ```
    pub fn start(self) -> CommunicatorServer {
        CommunicatorServer::new_pool("[::]:50051", Default::default(), self, CpuPool::new(4))
    }
}

impl Communicator for Server {
    /// Handle an *InitiateConversation* request. This will check the
    /// [`InitiateRequest`] to ensure the client is attempting to join a valid
    /// room, if so it will then add it to the room.
    ///
    /// [`InitiateRequest`]: message/struct.InitiateRequest.html
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

    /// Handle an *TerminateConversation* request. This will check the
    /// [`TerminateRequest`] to ensure the client is attempting to exit a valid
    /// room, if so it will then remove it to the room, provided it was already
    /// part of the room.
    ///
    /// [`TerminateRequest`]: message/struct.TerminateRequest.html
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
            reply.set_success(true);
            SingleResponse::completed(reply)
        }

    }

    /// Handle a *SendMessage* request. This is the main part of the
    /// communication protocol as it will open a bi-directional stream of
    /// messages. The [`MessageRequest`] received will be added to the
    /// conversation and the [`MessageReply`] will contain messages from other
    /// clients in the joined room.
    ///
    /// [`MessageRequest`]: message/struct.MessageRequest.html
    /// [`MessageReply`]: message/struct.MessageReply.html
    fn send_message(&self,
                    options: ::grpc::RequestOptions,
                    reqs: StreamingRequest<MessageRequest>)
                    -> StreamingResponse<MessageReply> {
        println!("Received a message stream opening request");
        let mut msgs = MsgIter::new();
        // First we should handle the input
        let mut requests: GrpcStreamSend<MessageRequest> = reqs.0;
        loop {
            match requests.deref_mut().poll().unwrap() {
                ::futures::Async::Ready(request) => {
                    println!("{:?}", request);
                    break;
                }
                ::futures::Async::NotReady => println!("Not ready"),
            }
        }


        // FIXME This was just for initial setup purposes
        // let mut msgs = vec![];
        // for i in 0..10 {
        //     let mut reply = Message::new();
        //     reply.set_content(format!("Message {}", i));
        //     reply.set_user(make_address());
        //     msgs.push(reply);
        // }
        let mut reply = MessageReply::new();
        reply.set_messages(msgs.next().unwrap());
        StreamingResponse::iter(vec![reply].into_iter())
    }
}

// FIXME This needs to go as soon as we can!
fn make_address() -> Address {
    let mut addr = Address::new();
    addr.set_address("remote".to_owned());
    addr
}

/// Type for the *rooms* that the server uses.
#[derive(Debug)]
struct MsgRoom {
    room: message::Room,
    users: Vec<message::Address>,
    messages: MsgIter,
}


impl MsgRoom {
    fn name(&self) -> &str {
        self.room.get_name()
    }

    fn from(name: &str) -> MsgRoom {
        let mut room = message::Room::new();
        room.set_name(name.to_owned());
        MsgRoom {
            room: room,
            users: vec![],
            messages: MsgIter::new(),
        }
    }
}

#[derive(Debug)]
struct MsgIter {
    messages: VecDeque<message::Message>,
}

impl MsgIter {
    fn new() -> MsgIter {
        MsgIter { messages: VecDeque::new() }
    }

    fn add(&mut self, msg: message::Message) {
        self.messages.push_back(msg);
    }
}

impl Iterator for MsgIter {
    type Item = message::Message;

    // If no messages return None, if some, take it from the front. Essentially
    // a queue so may be a better implementation
    fn next(&mut self) -> Option<message::Message> {
        self.messages.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::test_utils;
    use message_grpc::{Communicator, CommunicatorServer};
    use message::*;

    #[test]
    fn test_initiate_valid() {
        let server = super::Server::new();
        let response = server
            .initiate_conversation(::grpc::RequestOptions::new(),
                                   test_utils::default_initiate())
            .wait_drop_metadata()
            .unwrap();
        assert!(response.get_success());
    }
    #[test]
    fn test_terminate_valid() {
        let server = super::Server::new();
        let response = server
            .terminate_conversation(::grpc::RequestOptions::new(),
                                    test_utils::default_terminate())
            .wait_drop_metadata()
            .unwrap();
        assert!(response.get_success());
    }
}
