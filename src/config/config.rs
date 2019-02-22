use std::collections::HashMap;
use std::sync::Arc;
use serde_derive::{Serialize, Deserialize};
use std::ops::Deref;

#[derive(Debug, Serialize, Deserialize)]
pub struct DBRoute {
    pub engine: String,
    pub user: String,
    pub pass: String,
    pub addr: String,
    pub db: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct MemRoute {
    pub host: String,
    pub port: i32,
    pub db: i32,
    pub expire: i64,
    pub pass: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZKS {
    pub path: String,
    pub servers: Vec<String>,
    pub buffer: i32,
    pub multiple: i32,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct DataRoute {
    pub db: DBRoute,
    pub mem: Option<MemRoute>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Services {
    pub port: i32,
    data: HashMap<String, DBRoute>,
}

impl Services {
    pub fn new(config :Vec<u8>) -> Self {
        let data = config.as_slice();

//        let s = String::from_utf8(data).unwrap().as_str();
        serde_json::from_slice(data).unwrap()
    }
}


pub fn init(file: String) {
    println!("{}", file);
}