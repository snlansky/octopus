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
use dal::value::ConvertTo;
use dal::dao::DaoResult;
use dal::table::Field;

pub struct Mem {
    record: HashMap<String, Vec<String>>,
    conn: Arc<Mutex<Connection>>,
}

impl Mem {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Mem {
        Mem { record: HashMap::new(), conn }
    }

    fn get_conn(&self) -> Result<MutexGuard<Connection>, Error> {
        let conn = self.conn.lock()
            .map_err(|e| Error::CommonError { info: format!("get mem connection lock error: {:?}", e) })?;
        Ok(conn)
    }

    pub fn del(&mut self, tbl: Arc<Table>, mid: Vec<String>) -> Result<isize, Error> {
        let mut lua = LuaScript::new();
        lua.del(mid.clone());
        lua.srem(tbl.get_table_set_key(), mid);

        let conn = self.get_conn()?;
        lua.invoke(&conn).map_err(|e| Error::from(e))
    }

    pub fn load_update(&mut self, tbl: Arc<Table>, body: JsValue, db: Arc<Mutex<DB>>) -> Result<Vec<String>, Error> {
        let conditions = body.get("conditions").ok_or(Error::CommonError { info: "invalid json format".to_string() })?;
        let cond = conditions.as_object().ok_or(Error::CommonError { info: "invalid json format at token conditions".to_string() })?;
        let (pv_map, pk_match) = Self::match_pk(tbl.clone(), cond);

        let values = body.get("values").ok_or(Error::CommonError { info: "invalid json format".to_string() })?;
        let values = values.as_object().ok_or(Error::CommonError { info: "invalid json format at token values".to_string() })?;

        let conn = self.get_conn()?;
        if pk_match {
            let mid = tbl.get_model_key(&pv_map);
            if conn.exists(mid.clone())? {
                let mut lua = LuaScript::new();
                let vs: HashMap<String, String> = values.convert();
                lua.hmset(mid.clone(), vs);
                lua.expire(mid.clone(), 60 * 60);
                let _:isize = lua.invoke(&conn).map_err(|e| Error::from(e))?;
                return Ok(vec![mid]);
            }
        }

        let mut dao = Dao::new(tbl.clone(), DML::Select, conditions.clone());
        let rows = match dao.exec_sql(db)? {
            DaoResult::Rows(rows) => rows,
            _ => unreachable!(),
        };
        let mut lua = LuaScript::new();
        let mut mids = Vec::with_capacity(rows.len());
        for mut row in rows {
            let mid = tbl.get_model_key(&row);
            for (key, value) in values {
                row.insert(key.clone(), value.clone());
            }
            let vs: HashMap<String, String> = row.into_iter()
                .map(|(k, v)| (k, v.convert()))
                .collect::<HashMap<_, _>>();
            lua.hmset(mid.clone(), vs);
            lua.expire(mid.clone(), 60 * 60);
            mids.push(mid);
        }
        let _: isize = lua.invoke(&conn).map_err(|e| Error::from(e))?;
        Ok(mids)
    }

    pub fn load_find(&mut self, tbl: Arc<Table>, pv: HashMap<String, JsValue>, cond: JsValue, db: Arc<Mutex<DB>>, fields: &Vec<Field>) -> Result<JsValue, Error> {
        let mid = tbl.get_model_key(&pv);
        let conn = self.get_conn()?;
        let exist: bool = conn.exists(mid.clone())?;
        if !exist {
            let mut dao = Dao::new(tbl.clone(), DML::Select, cond);
            let rows = match dao.exec_sql(db.clone())? {
                DaoResult::Rows(rows) => rows,
                _ => unreachable!(),
            };
            if rows.len() == 0 {
                return Ok(JsValue::Array(Vec::new()));
            }
            let mut lua = LuaScript::new();
            let row0 = rows.get(0).unwrap().into_iter()
                .map(|(k, v)| {
                    let s: String = v.convert();
                    (k.clone(), s)
                })
                .collect::<HashMap<_, _>>();
            lua.hmset(mid.clone(), row0);
            lua.expire(mid.clone(), 60 * 60);
            lua.invoke(&conn);
        }


        let _: () = self.try_register_schema(tbl.clone(), &conn)?;

        let row;
        if fields.len() > 0 {
            row = self.get_value(mid, fields, &conn)?;
        } else {
            row = self.get_value(mid, tbl.get_fields(), &conn)?;
        }
        Ok(JsValue::Array(vec![row]))
    }

    pub fn match_pk(tbl: Arc<Table>, cond: &Map<String, JsValue>) -> (HashMap<String, JsValue>, bool) {
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
    pub fn try_register_schema(&self, tbl: Arc<Table>, con: &Connection) -> Result<(), Error> {
        let exist: bool = con.exists(tbl.get_table_schema_key())?;
        if exist {
            return Ok(());
        }
        tbl.register_schema(con).map_err(|e| Error::MemError(e))
    }

    fn get_value(&self, mid: String, fields: &Vec<Field>, con: &Connection) -> Result<JsValue, Error> {
        let fs = fields.iter().map(|f| f.name.clone()).collect::<Vec<_>>();
        let values: Vec<String> = con.hget(mid, fs)?;

        let mut fv = Map::new();
        for i in 0..values.len() {
            let field = &fields[i];
            let v = field.get_value(&values[i])?;
            fv.insert(field.name.clone(), v);
        }
        Ok(JsValue::Object(fv))
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
        let res = mem.del(Arc::new(table), vec!["block:TbTestModel:RoleGuid,TwoKey:0000009b790008004b64fb,3".to_string()]).unwrap();

        assert_eq!(res, 1);
    }

    #[test]
    fn test_mem_load_update() {
        let (table, mut mem,  db) = get_table_conn();
        let data = r##"{"conditions":{"RoleGuid__eq":"0000009b790008004b64fb","TwoKey__eq":"3","operator":"AND"},"values":{"CreateDate":"2017-00-00","CreateDatetime":"2017-00-00 09:16:55","CreateTime":"10:00:00","CreateTimestamp":"1"}}"##;
        let body: Value = serde_json::from_str(data).unwrap();

        let res = mem.load_update(Arc::new(table), body, Arc::new(Mutex::new(db))).unwrap();
        assert_eq!(res.len(), 1);
    }

    #[test]
    fn test_mem_load_find() {
        let (table, mut mem, db) = get_table_conn();
        let data = r##"{"RoleGuid__eq":"0000009b790008004b64fb","TwoKey__eq":3,"operator":"AND"}"##;
        let conditions: Value = serde_json::from_str(data).unwrap();

        let cond = conditions.clone();
        let cond = cond.as_object().unwrap();
        let table = Arc::new(table);
        let (pv_map, _) = Mem::match_pk(table.clone(), cond);
        let fields = vec![Field { name: "RoleGuid".to_string(), tpe: "varchar".to_string() }, Field { name: "TwoKey".to_string(), tpe: "int".to_string() }];
        let res = mem.load_find(table, pv_map, conditions, Arc::new(Mutex::new(db)), &fields).unwrap();
        println!("{}", res);
    }
}
