use zookeeper::{Acl, CreateMode, Watcher, WatchedEvent, ZooKeeper};
use zookeeper::recipes::cache::{PathChildrenCache, PathChildrenCacheEvent};
use std::time::Duration;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::thread;

struct LoggingWatcher;

impl Watcher for LoggingWatcher {
    fn handle(&self, e: WatchedEvent) {
        info!("zk->: {:?}", e)
    }
}

pub struct ServiceRegister {}

impl ServiceRegister {
    pub fn init(urls: &str, root_path: &str) {
        let listen = thread::spawn(move || {});

        let zk = ZooKeeper::connect(urls, Duration::from_secs(15), LoggingWatcher).unwrap();

        let zk_arc = Arc::new(zk);
        let mut pcc = PathChildrenCache::new(zk_arc.clone(), "/").unwrap();
        match pcc.start() {
            Err(e) => panic!(e),
            _ => info!("root path cache started"),
        }

        let (ev_tx, ev_rx) = channel();

        pcc.add_listener(move |e| ev_tx.send(e).unwrap());

        let root = root_path.to_string();
        thread::spawn(move || {
            for ev in ev_rx {
                match ev {
                    PathChildrenCacheEvent::ConnectionLost => panic!("zookeeper connection lost"),
                    PathChildrenCacheEvent::ChildRemoved(node) => {
                        println!("=>{}", node);
                    }
                    PathChildrenCacheEvent::ChildUpdated(node, data) => {
                        if node.eq(&root) {
                            info!("config update {:?}", data);
                        }
                    }
                    _ => ()
                }
            }
        });
    }
}