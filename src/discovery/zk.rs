use zookeeper::{Acl, CreateMode, Watcher, WatchedEvent, ZooKeeper};
use zookeeper::recipes::cache::{PathChildrenCache, PathChildrenCacheEvent};
use std::time::Duration;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::thread;
use zookeeper::ZkError;

struct LoggingWatcher;

impl Watcher for LoggingWatcher {
    fn handle(&self, e: WatchedEvent) {
        info!("zk->: {:?}", e)
    }
}

pub struct ServiceRegister {
    zk: Arc<ZooKeeper>,
}

impl ServiceRegister {
    pub fn new(urls: &str) -> Self {
        let listen = thread::spawn(move || {});

        let zk = ZooKeeper::connect(urls, Duration::from_secs(15), LoggingWatcher).unwrap();
        ServiceRegister { zk: Arc::new(zk) }
    }

    pub fn watch_data<>(&self, path: String, on_update: fn(Arc<Vec<u8>>)) -> Result<(), ZkError> {
        let mut pcc = PathChildrenCache::new(self.zk.clone(), "/").unwrap();
        let _ :() = pcc.start()?;

        let (ev_tx, ev_rx) = channel();
        pcc.add_listener(move |e| ev_tx.send(e).unwrap());

        thread::spawn(move || {
            for ev in ev_rx {
                match ev {
                    PathChildrenCacheEvent::ConnectionLost => panic!("zookeeper connection lost"),
                    PathChildrenCacheEvent::ChildRemoved(node) => {
                        println!("=>{}", node);
                    }
                    PathChildrenCacheEvent::ChildUpdated(node, data) => {
                        if node.eq(&path) {
                            info!("config update {:?}", data);
                        }
                    }
                    _ => ()
                }
            }
        });
        Ok(())
    }
}