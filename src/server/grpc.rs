use dal::add;
use dal::Support;
use grpc::RequestOptions;
use grpc::Server;
use grpc::SingleResponse;
use proto::orm::Request;
use proto::orm::Response;
use proto::orm_grpc::Orm;
use proto::orm_grpc::OrmServer;
use std::sync::Arc;
use std::sync::Mutex;

pub fn new(support: Arc<Mutex<Support>>) -> Server {
    let lock = support.lock().unwrap();
    let port = lock.port();
    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_port(port as u16);
    server.http.set_cpu_pool_threads(4);
    server.add_service(OrmServer::new_service_def(Handler::new(support.clone())));
    server.build().unwrap()
}

struct Handler {
    support: Arc<Mutex<Support>>,
}

impl Handler {
    pub fn new(support: Arc<Mutex<Support>>) -> Handler {
        Handler { support }
    }
}

impl Orm for Handler {
    fn add(&self, opt: RequestOptions, req: Request) -> SingleResponse<Response> {
        //        unimplemented!()
        let support = self.support.clone();
        let lock = support.try_lock();
        if lock.is_err() {
            return grpc::SingleResponse::err(grpc::Error::Panic(
                "get db source failed".to_string(),
            ));
        }
        let lock = lock.unwrap();

        // grpc::SingleResponse::err(grpc::Error::Panic("uri is null".to_string()))
        let uri = req.uri.as_ref();
        if uri.is_none() {
            return grpc::SingleResponse::err(grpc::Error::Panic("uri is null".to_string()));
        }
        let uri = uri.unwrap();

        let route = lock.data_route(uri.db.as_str());
        if route.is_none() {
            return grpc::SingleResponse::err(grpc::Error::Panic(format!("not fond db:{}", uri.db)));
        }
        let route = route.unwrap();



        grpc::SingleResponse::completed(Response::new())
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
