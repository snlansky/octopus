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

pub trait OrmRoute {
    fn add(&self, o: ::grpc::RequestOptions, p: super::orm::OneReq) -> ::grpc::SingleResponse<super::orm::Result>;

    fn remove(&self, o: ::grpc::RequestOptions, p: super::orm::OneReq) -> ::grpc::SingleResponse<super::orm::Result>;

    fn modify(&self, o: ::grpc::RequestOptions, p: super::orm::OneReq) -> ::grpc::SingleResponse<super::orm::Result>;

    fn find(&self, o: ::grpc::RequestOptions, p: super::orm::OneReq) -> ::grpc::SingleResponse<super::orm::Result>;

    fn transact(&self, o: ::grpc::RequestOptions, p: super::orm::OneReq) -> ::grpc::SingleResponse<super::orm::Result>;
}

// client

pub struct OrmRouteClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_Add: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::orm::OneReq, super::orm::Result>>,
    method_Remove: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::orm::OneReq, super::orm::Result>>,
    method_Modify: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::orm::OneReq, super::orm::Result>>,
    method_Find: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::orm::OneReq, super::orm::Result>>,
    method_Transact: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::orm::OneReq, super::orm::Result>>,
}

impl ::grpc::ClientStub for OrmRouteClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        OrmRouteClient {
            grpc_client: grpc_client,
            method_Add: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/ormroute.OrmRoute/Add".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Remove: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/ormroute.OrmRoute/Remove".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Modify: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/ormroute.OrmRoute/Modify".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Find: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/ormroute.OrmRoute/Find".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Transact: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/ormroute.OrmRoute/Transact".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl OrmRoute for OrmRouteClient {
    fn add(&self, o: ::grpc::RequestOptions, p: super::orm::OneReq) -> ::grpc::SingleResponse<super::orm::Result> {
        self.grpc_client.call_unary(o, p, self.method_Add.clone())
    }

    fn remove(&self, o: ::grpc::RequestOptions, p: super::orm::OneReq) -> ::grpc::SingleResponse<super::orm::Result> {
        self.grpc_client.call_unary(o, p, self.method_Remove.clone())
    }

    fn modify(&self, o: ::grpc::RequestOptions, p: super::orm::OneReq) -> ::grpc::SingleResponse<super::orm::Result> {
        self.grpc_client.call_unary(o, p, self.method_Modify.clone())
    }

    fn find(&self, o: ::grpc::RequestOptions, p: super::orm::OneReq) -> ::grpc::SingleResponse<super::orm::Result> {
        self.grpc_client.call_unary(o, p, self.method_Find.clone())
    }

    fn transact(&self, o: ::grpc::RequestOptions, p: super::orm::OneReq) -> ::grpc::SingleResponse<super::orm::Result> {
        self.grpc_client.call_unary(o, p, self.method_Transact.clone())
    }
}

// server

pub struct OrmRouteServer;


impl OrmRouteServer {
    pub fn new_service_def<H : OrmRoute + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/ormroute.OrmRoute",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/ormroute.OrmRoute/Add".to_string(),
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
                        name: "/ormroute.OrmRoute/Remove".to_string(),
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
                        name: "/ormroute.OrmRoute/Modify".to_string(),
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
                        name: "/ormroute.OrmRoute/Find".to_string(),
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
                        name: "/ormroute.OrmRoute/Transact".to_string(),
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
