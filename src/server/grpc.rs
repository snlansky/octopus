use proto::orm_grpc::OrmServer;
use proto::orm_grpc::Orm;
use grpc::RequestOptions;
use proto::orm::Request;
use grpc::SingleResponse;
use proto::orm::Response;
use dal::Support;
use std::sync::Arc;
use std::sync::Mutex;
use grpc::Server;

pub fn new(support: Arc<Mutex<Support>>) -> Server {
    let lock = support.lock().unwrap();
    let port = lock.port();
    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_port(port as u16);
    server.http.set_cpu_pool_threads(4);
    server.add_service(OrmServer::new_service_def(Handler::new()));
    server.build().unwrap()
}

struct Handler {}

impl Handler {
    pub fn new() -> Handler {
        Handler {}
    }
}

impl Orm for Handler {
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

