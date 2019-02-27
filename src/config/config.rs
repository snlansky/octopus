use std::collections::HashMap;
use serde_derive::{Serialize, Deserialize};


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
pub struct DataRoute {
    pub db: DBRoute,
    pub mem: Option<MemRoute>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Services {
    pub port: i32,
    data: HashMap<String, DataRoute>,
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