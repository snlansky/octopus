use std::sync::Arc;
use std::sync::Mutex;

pub fn ArcMutex<T>(t :T) ->Arc<Mutex<T>> {
    Arc::new(Mutex::new(t))
}