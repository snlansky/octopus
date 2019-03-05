use std::sync::Arc;
use std::sync::Mutex;

pub fn arc_mutex<T>(t :T) ->Arc<Mutex<T>> {
    Arc::new(Mutex::new(t))
}