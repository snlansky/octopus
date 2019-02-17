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



use config::config::init;
use log::Level;

mod dal;
mod config;
mod discovery;


fn main() {
    env_logger::init();
    init("service.json".to_string());
    debug!("ok");
}

