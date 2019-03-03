use discovery::Register;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use std::sync::Arc;
use config::Services;

pub trait Provider {
    fn watch(&mut self) -> Services;
}


pub struct Config {
    root: String,
    sr: Arc<Register>,
    start: bool,
    tx: Arc<Mutex<Sender<()>>>,
    rx: Receiver<()>,
}

impl Config {
    pub fn new(path: &str, sr: Arc<Register>) -> Self {
        let (tx, rx) = channel();
        Config {
            root: path.to_string(),
            sr,
            start: false,
            tx: Arc::new(Mutex::new(tx)),
            rx,
        }
    }
}

impl Provider for Config {
    fn watch(&mut self) -> Services {
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
