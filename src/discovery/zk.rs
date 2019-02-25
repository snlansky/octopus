use zookeeper::Watcher;
use zookeeper::WatchedEvent;
use zookeeper::ZooKeeper;
use std::time::Duration;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::Mutex;
use zookeeper::ZkError;

struct LoggingWatcher;

impl Watcher for LoggingWatcher {
    fn handle(&self, e: WatchedEvent) {
        info!("zk->: {:?}", e)
    }
}

pub struct ServiceRegister {
    zk: ZooKeeper,
}

type EventCallBack=fn(&Vec<u8>) ->bool;

impl ServiceRegister {
    pub fn new(urls: &str) -> Self {
        let zk = ZooKeeper::connect(urls,
                                    Duration::from_secs(15),
                                    LoggingWatcher).unwrap();
        ServiceRegister {
            zk,
        }
    }

    pub fn watch_data(&mut self, path: String, on_update: EventCallBack)->Result<(), ZkError> {
        let (ev_tx, ev_rx) = channel();
        let arc_rx = Arc::new(Mutex::new(ev_tx));
        loop{
            let rx = arc_rx.clone();
            let (data, _) = self.zk.get_data_w(path.as_str(), move|f:WatchedEvent|{
               rx.lock().unwrap().send(f).unwrap();
            })?;
            if !on_update(&data) {
                break;
            };
            ev_rx.recv().unwrap();
        }
        Ok(())
    }

    fn split(path: String) -> (String, String) {
        if !path.contains("/") {
            ("/".to_string(), path)
        } else {
            let mut v: Vec<_> = path.split("/").collect();
            let mut v = v.into_iter()
                .filter(|&f| !f.eq(""))
                .collect::<Vec<_>>();
            let node = v.pop().unwrap();
            (format!("/{}", v.join("/")), node.to_string())
        }
    }
}