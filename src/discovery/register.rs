use zookeeper::Watcher;
use zookeeper::WatchedEvent;
use zookeeper::ZooKeeper;
use std::time::Duration;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::Mutex;
use zookeeper::ZkError;
use std::sync::mpsc::Sender;

struct LoggingWatcher;

impl Watcher for LoggingWatcher {
    fn handle(&self, e: WatchedEvent) {
        info!("zk->: {:?}", e)
    }
}

pub struct Register {
    pub zk: ZooKeeper,
}

impl Register {
    pub fn new(urls: &str) -> Self {
        let zk = ZooKeeper::connect(urls,
                                    Duration::from_secs(15),
                                    LoggingWatcher).unwrap();
        Register {
            zk,
        }
    }

    pub fn watch_data<F>(&self, path: &str, on_update: F) -> Result<(), ZkError>
        where F: Fn(&Vec<u8>) -> bool {
        let (ev_tx, ev_rx) = channel();
        let arc_tx = Arc::new(Mutex::new(ev_tx));
        loop {
            let tx = arc_tx.clone();
            let (data, _) = self.zk.get_data_w(path, move |f: WatchedEvent| {
                tx.lock().unwrap().send(f).unwrap();
            })?;
            if !on_update(&data) {
                break;
            };
            ev_rx.recv().unwrap();
        }
        Ok(())
    }

    pub fn get_data(&self, path: &str, sign: Arc<Mutex<Sender<WatchedEvent>>>) -> Result<Vec<u8>, ZkError> {
        let (data, _) = self.zk.get_data_w(path, move |f: WatchedEvent| {
            sign.lock().unwrap().send(f).unwrap();
        })?;
        Ok(data)
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