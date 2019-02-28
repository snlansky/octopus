use proto::orm_grpc::OrmClient;
use proto::orm_grpc::OrmServer;
use proto::orm_grpc::Orm;
use grpc::RequestOptions;
use proto::orm::Request;
use grpc::SingleResponse;
use proto::orm::Response;
use std::thread;

pub fn new(port: i32) {
    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_port(port as u16);
    server.http.set_cpu_pool_threads(4);
    server.add_service(OrmServer::new_service_def(RouterHandler::new()));

    let _server = server.build().unwrap();
    info!("greeter server started on port {}", port);
    loop {
        thread::park();
    }
}

struct RouterHandler {}

impl RouterHandler {
    pub fn new() -> RouterHandler {
        RouterHandler {}
    }
}

impl Orm for RouterHandler {
    fn add(&self, opt: RequestOptions, req: Request) -> SingleResponse<Response> {
        unimplemented!()
    }

    fn remove(&self, opt: RequestOptions, req: Request) -> SingleResponse<Response> {
        unimplemented!()
    }

    fn modify(&self, opt: RequestOptions, req: Request) -> SingleResponse<Response> {
        unimplemented!()
    }

    fn find(&self, opt: RequestOptions, req: Request) -> SingleResponse<Response> {
        unimplemented!()
    }

    fn transact(&self, opt: RequestOptions, req: Request) -> SingleResponse<Response> {
        unimplemented!()
    }
}

