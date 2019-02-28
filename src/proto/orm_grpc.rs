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

pub trait Orm {
    fn add(&self, o: ::grpc::RequestOptions, p: super::orm::Request) -> ::grpc::SingleResponse<super::orm::Response>;

    fn remove(&self, o: ::grpc::RequestOptions, p: super::orm::Request) -> ::grpc::SingleResponse<super::orm::Response>;

    fn modify(&self, o: ::grpc::RequestOptions, p: super::orm::Request) -> ::grpc::SingleResponse<super::orm::Response>;

    fn find(&self, o: ::grpc::RequestOptions, p: super::orm::Request) -> ::grpc::SingleResponse<super::orm::Response>;

    fn transact(&self, o: ::grpc::RequestOptions, p: super::orm::Request) -> ::grpc::SingleResponse<super::orm::Response>;
}

// client

pub struct OrmClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_Add: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::orm::Request, super::orm::Response>>,
    method_Remove: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::orm::Request, super::orm::Response>>,
    method_Modify: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::orm::Request, super::orm::Response>>,
    method_Find: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::orm::Request, super::orm::Response>>,
    method_Transact: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::orm::Request, super::orm::Response>>,
}

impl ::grpc::ClientStub for OrmClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        OrmClient {
            grpc_client: grpc_client,
            method_Add: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/protos.Orm/Add".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Remove: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/protos.Orm/Remove".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Modify: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/protos.Orm/Modify".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Find: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/protos.Orm/Find".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Transact: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/protos.Orm/Transact".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl Orm for OrmClient {
    fn add(&self, o: ::grpc::RequestOptions, p: super::orm::Request) -> ::grpc::SingleResponse<super::orm::Response> {
        self.grpc_client.call_unary(o, p, self.method_Add.clone())
    }

    fn remove(&self, o: ::grpc::RequestOptions, p: super::orm::Request) -> ::grpc::SingleResponse<super::orm::Response> {
        self.grpc_client.call_unary(o, p, self.method_Remove.clone())
    }

    fn modify(&self, o: ::grpc::RequestOptions, p: super::orm::Request) -> ::grpc::SingleResponse<super::orm::Response> {
        self.grpc_client.call_unary(o, p, self.method_Modify.clone())
    }

    fn find(&self, o: ::grpc::RequestOptions, p: super::orm::Request) -> ::grpc::SingleResponse<super::orm::Response> {
        self.grpc_client.call_unary(o, p, self.method_Find.clone())
    }

    fn transact(&self, o: ::grpc::RequestOptions, p: super::orm::Request) -> ::grpc::SingleResponse<super::orm::Response> {
        self.grpc_client.call_unary(o, p, self.method_Transact.clone())
    }
}

// server

pub struct OrmServer;


impl OrmServer {
    pub fn new_service_def<H : Orm + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/protos.Orm",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/protos.Orm/Add".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.add(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/protos.Orm/Remove".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.remove(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/protos.Orm/Modify".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.modify(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/protos.Orm/Find".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.find(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/protos.Orm/Transact".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.transact(o, p))
                    },
                ),
            ],
        )
    }
}
