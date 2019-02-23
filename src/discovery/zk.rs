use zookeeper::{Watcher, WatchedEvent, ZooKeeper};
use zookeeper::recipes::cache::{PathChildrenCache, PathChildrenCacheEvent};
use std::time::Duration;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::thread;
use zookeeper::ZkError;
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::thread::JoinHandle;
use std::ops::Deref;
use std::sync::Mutex;

struct LoggingWatcher;

impl Watcher for LoggingWatcher {
    fn handle(&self, e: WatchedEvent) {
        info!("zk->: {:?}", e)
    }
}

type EventCallBack = fn(&Vec<u8>);
type SafeValue = Arc<(String, Vec<u8>)>;

pub struct ServiceRegister {
    zk: ZooKeeper,
    on_update: HashMap<String, EventCallBack>,
    event_tx: Arc<Sender<SafeValue>>,
    event_rx: Arc<Receiver<SafeValue>>,
}


impl ServiceRegister {
    pub fn new(urls: &str) -> Arc<Mutex<Self>> {
        let listen = thread::spawn(move || {});

        let zk = ZooKeeper::connect(urls, Duration::from_secs(15), LoggingWatcher).unwrap();

        let (tx, rx) = channel();
        let sr = ServiceRegister {
            zk,
            on_update: HashMap::new(),
            event_tx: Arc::new(tx),
            event_rx: Arc::new(rx),
        };
        Arc::new(Mutex::new(sr))
    }

    pub fn watch_data(this :Arc<Mutex<Self>>, path: String, on_update: EventCallBack) -> Result<(), ZkError> {
        let (parent, node) = Self::split(path.clone());

        let sr = this.lock().unwrap();
        sr.on_update.insert(path.clone(), on_update);

        let mut pcc = PathChildrenCache::new(Arc::new(sr.zk), parent.as_str()).unwrap();
        let _: () = pcc.start()?;

        let rx = sr.event_tx.clone();
        pcc.add_listener(move |event| {
            match event {
                PathChildrenCacheEvent::ChildUpdated(child, data) => {
                    if child.eq(&node) {
                        let (value, _) = data.deref();
                        let v = Arc::new((child, value.clone()));
                        match rx.send(v) {
                            Err(err) => { error!("{} send event error:{:?}", path.clone(), err); }
                            _ => (),
                        }
                    }
                }
                _ => (),
            }
        });
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

    pub fn wait_stop(this: Arc<Mutex<Self>>) -> JoinHandle<()> {
        thread::spawn(move || {
            let rs = this.lock().unwrap();
            let rx = rs.event_rx.deref();
            for sv in rx {
                let (path, value) = sv.deref();
                if let Some(f) = rs.on_update.get(path) {
                    f(value);
                }
            }
        })
    }
}