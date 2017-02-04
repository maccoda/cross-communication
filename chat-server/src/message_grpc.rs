// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait Communicator {
    fn InitiateConversation(&self, p: super::message::ConversationControlRequest) -> ::grpc::result::GrpcResult<super::message::ConversationControlReply>;

    fn TerminateConversation(&self, p: super::message::ConversationControlRequest) -> ::grpc::result::GrpcResult<super::message::ConversationControlReply>;

    fn SendMessage(&self, p: super::message::MessageRequest) -> ::grpc::result::GrpcResult<super::message::MessageResponse>;

    fn UpdateConversation(&self, p: super::message::UpdateRequest) -> ::grpc::result::GrpcResult<super::message::UpdateResponse>;
}

pub trait CommunicatorAsync {
    fn InitiateConversation(&self, p: super::message::ConversationControlRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::message::ConversationControlReply>;

    fn TerminateConversation(&self, p: super::message::ConversationControlRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::message::ConversationControlReply>;

    fn SendMessage(&self, p: super::message::MessageRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::message::MessageResponse>;

    fn UpdateConversation(&self, p: super::message::UpdateRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::message::UpdateResponse>;
}

// sync client

pub struct CommunicatorClient {
    async_client: CommunicatorAsyncClient,
}

impl CommunicatorClient {
    pub fn new(host: &str, port: u16, tls: bool) -> ::grpc::result::GrpcResult<Self> {
        CommunicatorAsyncClient::new(host, port, tls).map(|c| {
            CommunicatorClient {
                async_client: c,
            }
        })
    }
}

impl Communicator for CommunicatorClient {
    fn InitiateConversation(&self, p: super::message::ConversationControlRequest) -> ::grpc::result::GrpcResult<super::message::ConversationControlReply> {
        ::futures::Future::wait(self.async_client.InitiateConversation(p))
    }

    fn TerminateConversation(&self, p: super::message::ConversationControlRequest) -> ::grpc::result::GrpcResult<super::message::ConversationControlReply> {
        ::futures::Future::wait(self.async_client.TerminateConversation(p))
    }

    fn SendMessage(&self, p: super::message::MessageRequest) -> ::grpc::result::GrpcResult<super::message::MessageResponse> {
        ::futures::Future::wait(self.async_client.SendMessage(p))
    }

    fn UpdateConversation(&self, p: super::message::UpdateRequest) -> ::grpc::result::GrpcResult<super::message::UpdateResponse> {
        ::futures::Future::wait(self.async_client.UpdateConversation(p))
    }
}

// async client

pub struct CommunicatorAsyncClient {
    grpc_client: ::grpc::client::GrpcClient,
    method_InitiateConversation: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::message::ConversationControlRequest, super::message::ConversationControlReply>>,
    method_TerminateConversation: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::message::ConversationControlRequest, super::message::ConversationControlReply>>,
    method_SendMessage: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::message::MessageRequest, super::message::MessageResponse>>,
    method_UpdateConversation: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::message::UpdateRequest, super::message::UpdateResponse>>,
}

impl CommunicatorAsyncClient {
    pub fn new(host: &str, port: u16, tls: bool) -> ::grpc::result::GrpcResult<Self> {
        ::grpc::client::GrpcClient::new(host, port, tls).map(|c| {
            CommunicatorAsyncClient {
                grpc_client: c,
                method_InitiateConversation: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/Communicator/InitiateConversation".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_TerminateConversation: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/Communicator/TerminateConversation".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_SendMessage: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/Communicator/SendMessage".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_UpdateConversation: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/Communicator/UpdateConversation".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
            }
        })
    }
}

impl CommunicatorAsync for CommunicatorAsyncClient {
    fn InitiateConversation(&self, p: super::message::ConversationControlRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::message::ConversationControlReply> {
        self.grpc_client.call_unary(p, self.method_InitiateConversation.clone())
    }

    fn TerminateConversation(&self, p: super::message::ConversationControlRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::message::ConversationControlReply> {
        self.grpc_client.call_unary(p, self.method_TerminateConversation.clone())
    }

    fn SendMessage(&self, p: super::message::MessageRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::message::MessageResponse> {
        self.grpc_client.call_unary(p, self.method_SendMessage.clone())
    }

    fn UpdateConversation(&self, p: super::message::UpdateRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::message::UpdateResponse> {
        self.grpc_client.call_unary(p, self.method_UpdateConversation.clone())
    }
}

// sync server

pub struct CommunicatorServer {
    async_server: CommunicatorAsyncServer,
}

struct CommunicatorServerHandlerToAsync {
    handler: ::std::sync::Arc<Communicator + Send + Sync>,
    cpupool: ::futures_cpupool::CpuPool,
}

impl CommunicatorAsync for CommunicatorServerHandlerToAsync {
    fn InitiateConversation(&self, p: super::message::ConversationControlRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::message::ConversationControlReply> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.InitiateConversation(p)
        })
    }

    fn TerminateConversation(&self, p: super::message::ConversationControlRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::message::ConversationControlReply> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.TerminateConversation(p)
        })
    }

    fn SendMessage(&self, p: super::message::MessageRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::message::MessageResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.SendMessage(p)
        })
    }

    fn UpdateConversation(&self, p: super::message::UpdateRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::message::UpdateResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.UpdateConversation(p)
        })
    }
}

impl CommunicatorServer {
    pub fn new<A : ::std::net::ToSocketAddrs, H : Communicator + Send + Sync + 'static>(addr: A, h: H) -> Self {
        let h = CommunicatorServerHandlerToAsync {
            cpupool: ::futures_cpupool::CpuPool::new_num_cpus(),
            handler: ::std::sync::Arc::new(h),
        };
        CommunicatorServer {
            async_server: CommunicatorAsyncServer::new(addr, h),
        }
    }
}

// async server

pub struct CommunicatorAsyncServer {
    grpc_server: ::grpc::server::GrpcServer,
}

impl CommunicatorAsyncServer {
    pub fn new<A : ::std::net::ToSocketAddrs, H : CommunicatorAsync + 'static + Sync + Send + 'static>(addr: A, h: H) -> Self {
        let service_definition = CommunicatorAsyncServer::new_service_def(h);
        CommunicatorAsyncServer {
            grpc_server: ::grpc::server::GrpcServer::new(addr, service_definition),
        }
    }

    pub fn new_service_def<H : CommunicatorAsync + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::server::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::server::ServerServiceDefinition::new(
            vec![
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/Communicator/InitiateConversation".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.InitiateConversation(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/Communicator/TerminateConversation".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.TerminateConversation(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/Communicator/SendMessage".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.SendMessage(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/Communicator/UpdateConversation".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.UpdateConversation(p))
                    },
                ),
            ],
        )
    }
}
