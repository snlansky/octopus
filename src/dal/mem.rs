use std::collections::HashMap;
use dal::error::Error;
use dal::lua::LuaScript;
use dal::table::Table;
use std::sync::Mutex;
use redis::Connection;
use redis::Commands;
use std::sync::Arc;
use std::sync::MutexGuard;
use config::config::MemRoute;
use serde_json::Map;
use serde_json::Value as JsValue;
use dal::db::DB;
use dal::dao::Dao;
use dal::dao::DML;
use std::rc::Rc;
use dal::value::ConvertTo;

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

    pub fn load_update(&mut self, tbl: Rc<Table>, body: JsValue, db: Arc<Mutex<DB>>) -> Result<isize, Error> {
        let conditions = body.get("conditions").ok_or(Error::CommonError { info: "invalid json format".to_string() })?;
        let cond = conditions.as_object().ok_or(Error::CommonError { info: "invalid json format at token conditions".to_string() })?;
        let (pv_map, pk_match) = Self::match_pk(&tbl, cond);

        let values = body.get("values").ok_or(Error::CommonError { info: "invalid json format".to_string() })?;
        let values = values.as_object().ok_or(Error::CommonError { info: "invalid json format at token values".to_string() })?;

        let conn = self.get_conn()?;
        if pk_match {
            let mid = tbl.get_model_key(&pv_map);
            if conn.exists(mid.clone())? {
                let mut lua = LuaScript::new();
                let vs: HashMap<String, String> = values.convert();
                lua.hmset(mid.clone(), vs);
                lua.expire(mid, 60 * 60);
                let res = lua.invoke(&conn).map_err(|e| Error::from(e))?;
                return Ok(res);
            }
        }

        let tbl1 = tbl.clone();
        let mut dao = Dao::new(tbl1, DML::Select, conditions.clone());
        let v = dao.exec_sql(db)?;
        let rows = v.as_array().ok_or(Error::CommonError { info: format!("JsValue: {} as array failed", v) })?;
        let mut lua = LuaScript::new();
        for row in rows {
            let mut row = row.as_object().ok_or(Error::CommonError { info: format!("JsValue: {} as object failed", row) })?.clone();
            let pv: HashMap<String, JsValue> = row.convert();
            let mid = tbl.get_model_key(&pv);
            for (key, value) in values {
                row.insert(key.clone(), value.clone());
            }
            let vs: HashMap<String, String> = row.convert();
            lua.hmset(mid.clone(), vs);
            lua.expire(mid, 60 * 60);
        }
        lua.invoke(&conn).map_err(|e| Error::from(e))
    }

    pub fn load_find(&mut self, tbl: Rc<Table>, pv: HashMap<String, JsValue>, cond: JsValue, db: Arc<Mutex<DB>>) -> Result<JsValue, Error> {
        let mut lua = LuaScript::new();
        let mid = tbl.get_model_key(&pv);
        let conn = self.get_conn()?;
        let exist: bool = conn.exists(mid.clone())?;
        if !exist {
            let mut dao = Dao::new(tbl.clone(), DML::Select, cond);
            let exec_res = dao.exec_sql(db.clone())?;
//            let row = exec_res.rows();
        }
        Ok(json!(1))
    }

    pub fn match_pk(tbl: &Table, cond: &Map<String, JsValue>) -> (HashMap<String, JsValue>, bool) {
        let pks = tbl.get_pks();
        let pv = pks.iter()
            .filter_map(|key| {
                cond.get(format!("{}__eq", key).as_str()).map(|f| (key.clone(), f.clone()))
            })
            .collect::<HashMap<_, _>>();
        let m = pv.len() == pks.len();
        (pv, m)
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

#[cfg(test)]
mod tests {
    use dal::mem::open_client;
    use config::config::MemRoute;
    use dal::mem::Mem;
    use std::sync::Arc;
    use std::sync::Mutex;
    use dal::table::Table;
    use dal::table::Field;
    use std::rc::Rc;
    use serde_json::Value;
    use config::config::DBRoute;
    use dal::db::open_db;
    use dal::db::DB;

    fn get_table_conn() -> (Table, Mem, DB) {
        let r = MemRoute {
            host: "www.snlan.top".to_string(),
            port: 6379,
            pass: "snlan".to_string(),
            expire: 60 * 60,
            db: 0,
        };
        let conn = open_client(r).unwrap();
        let mem = Mem::new(Arc::new(Mutex::new(conn)));
        let db = "block".to_string();
        let model = "TbTestModel".to_string();
        let pks = vec!["RoleGuid".to_string(), "TwoKey".to_string()];
        let fields = vec![Field { name: "RoleGuid".to_string(), tpe: "varchar".to_string() },
                          Field { name: "TwoKey".to_string(), tpe: "int".to_string() },
                          Field { name: "CreateTime".to_string(), tpe: "varchar".to_string() },
                          Field { name: "CreateDatetime".to_string(), tpe: "date".to_string() },
                          Field { name: "CreateDate".to_string(), tpe: "datetime".to_string() },
                          Field { name: "CreateTimestamp".to_string(), tpe: "int".to_string() },
        ];
        let table = Table::default(db, model, pks, fields);

        let dbr = DBRoute {
            engine: String::from("Mysql"),
            user: String::from("snlan"),
            pass: String::from("snlan"),
            addr: String::from("www.snlan.top"),
            db: String::from("block"),
        };
        let db = open_db(dbr).unwrap();
        (table, mem, db)
    }

    #[test]
    fn test_mem_del() {
        let (table, mut mem, _) = get_table_conn();
        let res = mem.del(table, vec!["block:TbTestModel:RoleGuid,TwoKey:0000009b790008004b64fb,3".to_string()]).unwrap();

        assert_eq!(res, 1);
    }

    #[test]
    fn test_mem_bulk_update() {
        let (table, mut mem, mut db) = get_table_conn();
        let data = r##"{"conditions":{"RoleGuid__eq":"0000009b790008004b64fb","TwoKey__eq":"3","operator":"AND"},"values":{"CreateDate":"2017-00-00","CreateDatetime":"2017-00-00 09:16:55","CreateTime":"10:00:00","CreateTimestamp":"1"}}"##;
        let body: Value = serde_json::from_str(data).unwrap();

        let res = mem.load_update(Rc::new(table), body, Arc::new(Mutex::new(db))).unwrap();
        assert_eq!(res, 1);
    }
}
