use dal::add;
use dal::Support;
use error::Error;
use error::Error::CommonError;
use grpc::RequestOptions;
use grpc::Server;
use grpc::SingleResponse;
use proto::orm::Request;
use proto::orm::Response;
use proto::orm_grpc::Orm;
use proto::orm_grpc::OrmServer;
use std::panic::catch_unwind;
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

        let uri = req.uri.as_ref();
        let uri = if uri.is_some() {
            uri.unwrap()
        } else {
            return panic_error("uri is null".to_string());
        };

        let route = lock.data_route(uri.db.as_str());
        let route = if route.is_some() {
            route.unwrap()
        } else {
            return panic_error(format!("not fond db:{}", uri.db));
        };

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

impl From<Error> for grpc::SingleResponse<Response> {
    fn from(e: Error) -> Self {
        match e {
            Error::CommonError { info: e } => grpc::SingleResponse::err(grpc::Error::Panic(e)),
            _ => grpc::SingleResponse::err(grpc::Error::Panic("Internal error".to_string())),
        }
    }
}

fn panic_error(s: String) -> SingleResponse<Response> {
    grpc::SingleResponse::err(grpc::Error::Panic(s))
}
