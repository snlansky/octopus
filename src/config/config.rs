use std::collections::HashMap;
use std::sync::Arc;
use serde_derive::{Serialize, Deserialize};
use discovery::zk::ServiceRegister;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use serde_json::Error;
use std::sync::mpsc::Sender;

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

pub struct Provider {
//    tx: Sender<Vec<u8>>,
    rx: Receiver<Vec<u8>>,
}

impl Provider {
    pub fn new(path: &str, sr: &ServiceRegister) -> Self {
        let (tx, rx) = channel();

        sr.watch_data(path, move|f|{
            tx.send(f.clone()).unwrap();
            true
        }).unwrap();
        Provider{rx}
    }

    pub fn watch(&self) -> Services {
        let data = self.rx.recv().unwrap();
        let res: Result<Services, Error> = serde_json::from_slice(data.as_slice());

        match res {
            Ok(s) => s,
            _ => {
                error!("unmarshal json error");
                self.watch()
            }
        }
    }
}