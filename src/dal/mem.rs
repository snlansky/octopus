use std::collections::HashMap;
use dal::error::Error;
use dal::lua::LuaScript;
use dal::table::Table;
use std::sync::Mutex;
use redis::Connection;
use std::sync::Arc;
use std::sync::MutexGuard;
use config::config::MemRoute;

pub struct Mem {
    record: HashMap<String, Vec<String>>,
    conn: Arc<Mutex<Connection>>,
}

impl Mem {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Mem {
        Mem { record: HashMap::new(), conn }
    }


    fn get_conn(&mut self) -> Result<MutexGuard<Connection>, Error> {
        let conn = self.conn.lock()
            .map_err(|e| Error::CommonError { info: format!("get mem lock error: {:?}", e) })?;
        Ok(conn)
    }

    pub fn del(&mut self, tbl: Table, mid: Vec<String>) -> Result<isize, Error> {
        let mut lua = LuaScript::new();
        lua.del(mid.clone());
        lua.srem(tbl.get_table_set_key(), mid);

        let conn = self.get_conn()?;
        lua.invoke(&conn).map_err(|e| Error::from(e))
    }

    // 在cache中注册模式
    pub fn register_schema(&self, tbl: &Table) -> Result<(), Error> {
        Ok(())
    }
}


pub fn open_client(route: MemRoute) -> Result<Connection, Error> {
    let url = format!("redis://:{}@{}:{}/", route.pass, route.host, route.port);
    let client = redis::Client::open(url.as_str())?;
    client.get_connection().map_err(|e| Error::from(e))
}
