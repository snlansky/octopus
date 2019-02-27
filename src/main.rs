extern crate redis;
extern crate mysql;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;
extern crate core;
extern crate zookeeper;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate threadpool;
extern crate clap;
//extern crate protobuf;
//extern crate grpc;

use clap::App;
use discovery::Register;
use std::thread::sleep;
use std::time::Duration;
use clap::Arg;
use std::sync::Arc;
use config::Config;
use config::Provider;
use dal::Support;


mod dal;
mod config;
mod discovery;
mod proto;

fn main() {
    env_logger::init();

    let matches  = App::new("octopus")
        .version("1.0")
        .author("snlan@live.cn")
        .about("data bus")
        .arg(Arg::with_name("cluster")
            .short("c")
            .long("cluster")
            .value_name("ADDRESS")
            .help("The zookeeper cluster address.")
            .takes_value(true))
        .arg(Arg::with_name("path")
            .short("p")
            .long("path")
            .help("The config path at zookeeper.")
            .takes_value(true))
        .get_matches();

    let cluster = matches.value_of("cluster").unwrap_or("www.snlan.top:2181,www.snlan.top:2182,www.snlan.top:2183");
    let path = matches.value_of("path").unwrap_or("/dal_orm_release");

    info!("{} {}", cluster, path);

    let mut arc_register = Arc::new(Register::new(cluster));

    let mut provider = Config::new(path, arc_register.clone());

    let support = Support::new(arc_register, provider);

    sleep(Duration::from_secs(100));
}
