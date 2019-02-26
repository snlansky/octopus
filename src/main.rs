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

use clap::App;
use discovery::zk::ServiceRegister;
use std::thread::sleep;
use std::time::Duration;
use config::config::Provider;
use clap::Arg;
use std::sync::Arc;


mod dal;
mod config;
mod discovery;

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

    let mut sr = ServiceRegister::new(cluster);

    let mut provider = Provider::new(path, Arc::new(sr));

    loop {
        let s = provider.watch();
        println!("-->{:?}", s);
    }

    sleep(Duration::from_secs(100));
}
