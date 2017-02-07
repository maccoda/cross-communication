extern crate chat_server;
extern crate grpc;

use std::thread;

use grpc::result::GrpcResult;
use grpc::error::GrpcError;

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
        let unused_id: Vec<&ConversationId> =
            self.conversations.iter().filter(|x| !x.used).collect();
        if unused_id.len() == 0 {
            self.conversations.push(ConversationId {
                id: self.conversations.len() as u32,
                used: true,
                receipient: receipient,
            });
            self.conversations.len() as u32
        } else {
            unused_id[0].used = true;
            unused_id[0].id
        }
    }
}

impl Communicator for CommunicatorImpl {
    fn InitiateConversation(&self,
                            req: ConversationControlRequest)
                            -> GrpcResult<ConversationControlReply> {
        let mut reply = ConversationControlReply::new();
        println!("Received an initiate command from {:?}", req.get_address());
        println!("They wish to connect with {:?}", req.get_receipient());
        reply.set_conversationId(self.get_next_id(req.get_receipient().to_owned()));
        println!("Giving a conversation id of {:?}",
                 reply.get_conversationId());
        reply.set_success(true);
        Ok(reply)
    }

    fn TerminateConversation(&self,
                             req: ConversationControlRequest)
                             -> GrpcResult<ConversationControlReply> {
        let mut reply = ConversationControlReply::new();
        println!("Received a terminate command from {:?}", req.get_address());
        println!("They wish to end their conversation with {:?}",
                 req.get_receipient());
        // First check that the conversation can be ended
        let matches =
            self.conversations.iter().filter(|x| x.eq(req.get_conversationId())).collect();
        if matches != 1 {
            Err(grpc::error::GrpcError::Other(&format!("Conversation not yet open. Incorrect \
                                                       conversation ID {:?}.",
                                                       req.get_conversationId())))
        } else {
            matches[0].clear();
            Ok(reply)
        }

    }

    fn SendMessage(&self, req: MessageRequest) -> GrpcResult<MessageResponse> {
        Ok(MessageResponse::new())
    }

    fn UpdateConversation(&self, req: UpdateRequest) -> ::grpc::result::GrpcResult<UpdateResponse> {
        Ok(UpdateResponse::new())
    }
}
#[allow(unused_variables)]
fn main() {
    // Create the server, need unused variable so doesn't get disposed of
    let server = CommunicatorServer::new("[::]:50051", CommunicatorImpl::new());
    loop {
        thread::park();
    }
}
