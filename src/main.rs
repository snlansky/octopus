extern crate redis;
extern crate mysql;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate failure;
extern crate core;
extern crate zookeeper;

use serde_json::Value;
use config::config::init;
use config::config::ConfigType;

mod dal;
mod config;
mod service_discovery;


fn main() {
    init("service.json".to_string(), ConfigType::Json);
}

