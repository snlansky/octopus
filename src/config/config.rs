use std::collections::HashMap;
use std::sync::Arc;
use serde_derive::{Serialize, Deserialize};
use discovery::zk::ServiceRegister;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use serde_json::Error;
use std::sync::mpsc::Sender;
use std::thread;
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
pub struct DBRoute {
    pub engine: String,
    pub user: String,
    pub passwd: String,
    pub address: String,
    pub port: i32,
    pub name: String,
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
    data: HashMap<String, DataRoute>,
}

pub struct Provider {
    root: String,
    sr: Arc<ServiceRegister>,
    start: bool,
    tx: Arc<Mutex<Sender<()>>>,
    rx: Receiver<()>,

}

impl Provider {
    pub fn new(path: &str, sr: Arc<ServiceRegister>) -> Self {
        let (tx, rx) = channel();
        Provider {
            root: path.to_string(),
            sr,
            start: false,
            tx: Arc::new(Mutex::new(tx)),
            rx,
        }
    }

    pub fn watch(&mut self) -> Services {
        let sr = self.sr.clone();
        if self.start {
            self.rx.recv().unwrap();
        }

        let tx = self.tx.clone();
        let (data, _) = sr.zk.get_data_w(self.root.as_str(), move |f| {
            tx.lock().unwrap().send(()).unwrap();
        }).unwrap();

        self.start = true;
        match serde_json::from_slice(data.as_slice()) {
            Ok(s) => {
                s
            }
            _ => {
                error!("unmarshal json error");
                self.watch()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use config::config::DataRoute;
    use config::config::MemRoute;
    use config::config::DBRoute;
    use std::collections::HashMap;
    use config::config::Services;

    #[test]
    fn marhshal() {
        let db = DataRoute {
            db: DBRoute {
                engine: "mysql".to_string(),
                user: "snlan".to_string(),
                passwd: "snlan".to_string(),
                address: "www.snlan.top".to_string(),
                port: 3306,
                name: "block".to_string(),

            },
            mem: Some(MemRoute {
                host: "www.snlan.top".to_string(),
                port: 6379,
                db: 0,
                expire: 3600,
                pass: "snlan".to_string(),
            }),
        };
        let mut db_map = HashMap::new();
        db_map.insert("block".to_string(), db);

        let s = Services {
            port: 8081,
            data: db_map,
        };
        let j = serde_json::to_string(&s).unwrap();
        println!("{:?}", j);
    }

    #[test]
    fn unmarshal() {
        let data = r##"{
    "port": 8000,
    "data": {
        "block": {
            "db": {
                "engine": "mysql",
                "user": "snlan",
                "passwd": "snlan",
                "address": "www.snlan.top",
                "port": 3306,
                "name": "block"
            },
            "mem": {
                "host": "www.snlan.top",
                "port": 6379,
                "db": 0,
                "expire": 3600,
                "pass": "snlan"
            }
        }
    }
}"##;
        let s: Services = serde_json::from_str(data).unwrap();
        println!("{:?}", s);
    }
}
