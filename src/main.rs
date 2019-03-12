extern crate mysql;
extern crate redis;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate serde_derive;
#[macro_use]
extern crate failure;
extern crate core;
extern crate zookeeper;
#[macro_use]
extern crate log;
extern crate clap;
extern crate crossbeam;
extern crate env_logger;
extern crate grpc;
extern crate protobuf;
extern crate threadpool;

use clap::App;
use clap::Arg;
use config::Provider;
use dal::Support;
use discovery::Register;
use std::sync::Arc;
use std::thread;

mod config;
mod dal;
mod discovery;
mod error;
mod proto;
mod server;

fn init() {
    use std::env;
    let var = env::var("RUST_LOG");
    if var.is_err() || var.unwrap().is_empty() {
        env::set_var("RUST_LOG", "info");
    }
}

fn main() {
    init();
    env_logger::init();

    let matches = App::new("octopus")
        .version("1.0")
        .author("snlan@live.cn")
        .about("data bus")
        .arg(
            Arg::with_name("cluster")
                .short("c")
                .long("cluster")
                .value_name("ADDRESS")
                .help("The zookeeper cluster address.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("path")
                .short("p")
                .long("path")
                .help("The config path at zookeeper.")
                .takes_value(true),
        )
        .get_matches();

    let cluster = matches
        .value_of("cluster")
        .unwrap_or("www.snlan.top:2181,www.snlan.top:2182,www.snlan.top:2183");
    let path = matches.value_of("path").unwrap_or("/dal_orm_release");

    info!("{} {}", cluster, path);

    let arc_register = Arc::new(Register::new(cluster));
    let provider = Provider::new(path, arc_register.clone());
    let pool = threadpool::ThreadPool::new(4);
    let support = Support::new(arc_register, provider, &pool);
    let server = server::new(support);

    info!("grpc server started on {}", server.local_addr());
    loop {
        thread::park();
    }
}
