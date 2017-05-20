extern crate chat_server;
extern crate grpc;
extern crate futures;
extern crate futures_cpupool;

use std::thread;

use futures_cpupool::CpuPool;

use grpc::{SingleResponse, StreamingResponse, StreamingRequest};

use chat_server::message_grpc::*;
use chat_server::message::*;

#[derive(PartialEq)]
struct ConversationId {
    id: u32,
    used: bool,
    receipient: String,
}

impl ConversationId {
    fn clear(&mut self) {
        self.used = false;
        self.receipient = String::new();
    }
}
struct CommunicatorImpl {
    // NOTE This is just the basic implementation, a map or some sorted structure may be better
    // NOTE Would also need to have access to the database of conversation history
    conversations: Vec<ConversationId>,
}

impl CommunicatorImpl {
    fn new() -> CommunicatorImpl {
        CommunicatorImpl { conversations: vec![] }
    }
    fn get_next_id(&mut self, receipient: String) -> u32 {
        // let unused_id: Vec<&ConversationId> =
        //     self.conversations.iter().filter(|x| !x.used).collect();
        // if unused_id.len() == 0 {
        //     self.conversations.push(ConversationId {
        //         id: self.conversations.len() as u32,
        //         used: true,
        //         receipient: receipient,
        //     });
        //     self.conversations.len() as u32
        // } else {
        //     unused_id[0].used = true;
        //     unused_id[0].id
        // }
        0
    }
}

impl Communicator for CommunicatorImpl {
    fn initiate_conversation(&self, options: ::grpc::RequestOptions, req: InitiateRequest) -> SingleResponse<InitiateReply> {
        let mut reply = InitiateReply::new();
        println!("Received an initiate command from {:?}", req.get_address());
        println!("They wish to connect with {:?}", req.get_receipient());
        // reply.set_conversationId(self.get_next_id(req.get_receipient().get_name().to_owned()));
        println!("Giving a conversation id of {:?}",
                 reply.get_conversationId());
        reply.set_success(true);
        SingleResponse::completed(reply)
    }

    fn terminate_conversation(&self, options: ::grpc::RequestOptions, req: TerminateRequest) -> SingleResponse<TerminateReply> {
        let mut reply = TerminateReply::new();
        println!("Received a terminate command from {:?}", req.get_address());
        let req_id = req.get_conversationID();
        println!("They wish to end their conversation with {:?}", req_id);
        // First check that the conversation can be ended
        let matches: Vec<&ConversationId> =
            self.conversations.iter().filter(|x| x.id == req_id).collect();
        if matches.len() != 1 {
            SingleResponse::err(grpc::error::Error::Other("Conversation not yet open. Incorrect \
                                                       conversation ID"))
        } else {
            // matches[0].clear();
            SingleResponse::completed(reply)
        }

    }
    fn send_message(&self, options: ::grpc::RequestOptions,
                   reqs: StreamingRequest<MessageRequest>)
                   -> StreamingResponse<MessageReply> {
        // FIXME Unsure of how to make mock of the iterator
        StreamingResponse::completed(vec![MessageReply::new()])
    }
}
#[allow(unused_variables)]
fn main() {
    // Create the server, need unused variable so doesn't get disposed of
    let server = CommunicatorServer::new_pool("[::]:50051", Default::default(), CommunicatorImpl::new(), CpuPool::new(4));
    loop {
        thread::park();
    }
}
