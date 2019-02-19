extern crate redis;
extern crate mysql;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate failure;
extern crate core;
extern crate zookeeper;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate threadpool;
extern crate clap;


use config::config::init;
use clap::{Arg, App, SubCommand};


mod dal;
mod config;
mod discovery;

fn main() {
    env_logger::init();
    init("service.json".to_string());
    debug!("ok");

    let matches = App::new("octopus")
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

    let cluster = matches.value_of("cluster").unwrap_or("127.0.0.1:2181");
    let path = matches.value_of("path").unwrap_or("/dal_orm_release");
    info!("{} {}", cluster, path);
}
