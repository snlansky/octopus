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

pub fn new(support: Arc<Support>) -> Server {
    let port = support.port();
    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_port(port as u16);
    server.http.set_cpu_pool_threads(4);
    server.add_service(OrmServer::new_service_def(Handler::new(support.clone())));
    server.build().unwrap()
}

struct Handler {
    support: Arc<Support>,
}

impl Handler {
    pub fn new(support: Arc<Support>) -> Handler {
        Handler { support }
    }
}

impl Orm for Handler {
    fn add(&self, opt: RequestOptions, req: Request) -> SingleResponse<Response> {
        //        unimplemented!()
        let support = self.support.clone();

        let uri = match req.uri.as_ref() {
            Some(uri) => uri,
            None => {
                return panic_string("uri is null".to_string());
            }
        };

        let route = support.data().lock();

        let route = match route {
            Ok(data) => {
                data
            }
            Err(err) => {
                return panic_string(format!("lock db {} failed", uri.db));
            }
        };

        let route = match route.get(&uri.db) {
            Some(r) => r,
            None => {
                return panic_string(format!("not fond db:{}", uri.db));
            }
        };

        let res = add(uri, route, json!(req.body));

        match res {
            Ok(v) => {
                let mut resp = Response::new();
                resp.set_content(v.to_string().as_bytes().to_vec());
                grpc::SingleResponse::completed(resp)
            }
            Err(e) => grpc::SingleResponse::from(e),
        }
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

fn panic_error(e: Error) -> SingleResponse<Response> {
    match e {
        Error::CommonError { info: e } => grpc::SingleResponse::err(grpc::Error::Panic(e)),
        _ => grpc::SingleResponse::err(grpc::Error::Panic("Internal error".to_string())),
    }
}

fn panic_string(s: String) -> SingleResponse<Response> {
    grpc::SingleResponse::err(grpc::Error::Panic(s))
}
