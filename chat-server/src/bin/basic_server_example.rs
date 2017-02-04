extern crate chat_server;
extern crate grpc;

use std::thread;

use grpc::result::GrpcResult;

use chat_server::message_grpc::*;
use chat_server::message::*;

struct CommunicatorImpl;

impl Communicator for CommunicatorImpl {
    fn InitiateConversation(&self,
                            req: ConversationControlRequest)
                            -> GrpcResult<ConversationControlReply> {
        let mut reply = ConversationControlReply::new();
        println!("Received an initiate command from {:?}", req.get_address());
        println!("They wish to connect with {:?}", req.get_receipient());
        reply.set_conversationId(5);
        println!("Giving a conversation id of {:?}",
                 reply.get_conversationId());
        reply.set_success(true);
        Ok(reply)
    }

    fn TerminateConversation(&self,
                             req: ConversationControlRequest)
                             -> GrpcResult<ConversationControlReply> {
        Ok(ConversationControlReply::new())
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
    let server = CommunicatorServer::new("[::]:50051", CommunicatorImpl);
    loop {
        thread::park();
    }
}
