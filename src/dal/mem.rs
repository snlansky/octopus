use config::MemRoute;
use dal::dao::Dao;
use dal::dao::DaoResult;
use dal::dao::DML;
use dal::db::DB;
use dal::lua::LuaScript;
use dal::table::Field;
use dal::table::Table;
use dal::value::ConvertTo;
use error::Error;
use redis::Commands;
use redis::Connection;
use serde_json::Map;
use serde_json::Value as JsValue;
use std::collections::HashMap;
use std::sync::Arc;

pub struct Mem {
    con: Connection,
}

impl Mem {
    pub fn new(con: Connection) -> Self {
        Mem { con }
    }

    pub fn instance(route: &MemRoute) -> Result<Self, Error> {
        let con = open_client(route)?;
        Ok(Mem { con })
    }

    pub fn conn(&self) -> &Connection {
        &self.con
    }
}

pub struct MemContext<'a> {
    record: HashMap<String, Vec<String>>,
    mem: &'a Mem,
}

impl <'a>MemContext<'a> {
    pub fn new(mem: &'a Mem) -> Self {
        MemContext {
            record: HashMap::new(),
            mem,
        }
    }

    pub fn del(&mut self, tbl: &Table, mid: Vec<String>) -> Result<isize, Error> {
        let mut lua = LuaScript::new();
        lua.del(mid.clone());
        lua.srem(tbl.get_table_set_key(), mid);
        lua.invoke(self.mem.conn()).map_err(Error::from)
    }

    pub fn load_update(
        &mut self,
        tbl: &Table,
        body: JsValue,
        db: &DB,
    ) -> Result<Vec<String>, Error> {
        let conditions = body.get("conditions").ok_or(Error::CommonError {
            info: "invalid json format".to_string(),
        })?;
        let cond = conditions.as_object().ok_or(Error::CommonError {
            info: "invalid json format at token conditions".to_string(),
        })?;
        let (pv_map, pk_match) = Self::match_pk(tbl, cond);

        let values = body.get("values").ok_or(Error::CommonError {
            info: "invalid json format".to_string(),
        })?;
        let values = values.as_object().ok_or(Error::CommonError {
            info: "invalid json format at token values".to_string(),
        })?;

        if pk_match {
            let mid = tbl.get_model_key(&pv_map);
            if self.mem.conn().exists(mid.clone())? {
                let mut lua = LuaScript::new();
                let vs: HashMap<String, String> = values.convert();
                lua.hmset(mid.clone(), vs);
                lua.expire(mid.clone(), 60 * 60);
                let _: isize = lua.invoke(self.mem.conn()).map_err(Error::from)?;
                return Ok(vec![mid]);
            }
        }

        let mut dao = Dao::new(tbl, DML::Select, conditions.clone());
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
            let vs: HashMap<String, String> = row
                .into_iter()
                .map(|(k, v)| (k, v.convert()))
                .collect::<HashMap<_, _>>();
            lua.hmset(mid.clone(), vs);
            lua.expire(mid.clone(), 60 * 60);
            mids.push(mid);
        }
        let _: isize = lua.invoke(self.mem.conn()).map_err(Error::from)?;
        Ok(mids)
    }

    pub fn load_find(
        &mut self,
        tbl: &Table,
        pv: HashMap<String, JsValue>,
        cond: JsValue,
        db: &DB,
        fields: &[Field],
    ) -> Result<JsValue, Error> {
        let mid = tbl.get_model_key(&pv);
//        let mem = self.get_mem()?;
        let exist: bool = self.mem.conn().exists(mid.clone())?;
        if !exist {
            let mut dao = Dao::new(tbl, DML::Select, cond);
            let rows = match dao.exec_sql(db)? {
                DaoResult::Rows(rows) => rows,
                _ => unreachable!(),
            };
            if rows.is_empty() {
                return Ok(JsValue::Array(Vec::new()));
            }
            let mut lua = LuaScript::new();
            let row0 = (&rows[0])
                .iter()
                .map(|(k, v)| {
                    let s: String = v.convert();
                    (k.clone(), s)
                })
                .collect::<HashMap<_, _>>();
            lua.hmset(mid.clone(), row0);
            lua.expire(mid.clone(), 60 * 60);
            lua.invoke(self.mem.conn())?;
        }

        self.try_register_schema(tbl)?;

        let row = if fields.is_empty() {
            self.get_value(mid, tbl.get_fields())?
        } else {
            self.get_value(mid, fields)?
        };
        Ok(JsValue::Array(vec![row]))
    }

    pub fn match_pk(
        tbl: &Table,
        cond: &Map<String, JsValue>,
    ) -> (HashMap<String, JsValue>, bool) {
        let pks = tbl.get_pks();
        let pv = pks
            .iter()
            .filter_map(|key| {
                cond.get(format!("{}__eq", key).as_str())
                    .map(|f| (key.clone(), f.clone()))
            })
            .collect::<HashMap<_, _>>();
        let m = pv.len() == pks.len();
        (pv, m)
    }

    // 在cache中注册模式
    pub fn try_register_schema(&self, tbl: &Table) -> Result<(), Error> {
        let exist: bool = self.mem.conn().exists(tbl.get_table_schema_key())?;
        if exist {
            return Ok(());
        }
        tbl.register_schema(self.mem.conn()).map_err(Error::MemError)
    }

    fn get_value(&self, mid: String, fields: &[Field]) -> Result<JsValue, Error> {
        let fs = fields.iter().map(|f| f.name.clone()).collect::<Vec<_>>();
        let values: Vec<String> = self.mem.conn().hget(mid, fs)?;

        let mut fv = Map::new();
        for i in 0..values.len() {
            let field = &fields[i];
            let v = field.get_value(&values[i])?;
            fv.insert(field.name.clone(), v);
        }
        Ok(JsValue::Object(fv))
    }
}

pub fn open_client(route: &MemRoute) -> Result<Connection, Error> {
    let url = format!("redis://:{}@{}:{}/", route.pass, route.host, route.port);
    let client = redis::Client::open(url.as_str())?;
    client.get_connection().map_err(Error::from)
}
//
//#[cfg(test)]
//mod tests {
//    use config::DBRoute;
//    use config::MemRoute;
//    use dal::db::open_db;
//    use dal::db::DB;
//    use dal::mem::open_client;
//    use dal::mem::Mem;
//    use dal::mem::MemContext;
//    use dal::table::Field;
//    use dal::table::Table;
//    use serde_json::Value;
//    use std::sync::Arc;
//    use std::sync::Mutex;
//
//    fn get_table_conn() -> (Table, MemContext, DB) {
//        let r = MemRoute {
//            host: "www.snlan.top".to_string(),
//            port: 6379,
//            pass: "snlan".to_string(),
//            expire: 60 * 60,
//            db: 0,
//        };
//        let conn = open_client(&r).unwrap();
//
//        let mem = MemContext::new(Arc::new(Mutex::new(Mem::new(conn))));
//        let db = "block".to_string();
//        let model = "TbTestModel".to_string();
//        let pks = vec!["RoleGuid".to_string(), "TwoKey".to_string()];
//        let fields = vec![
//            Field {
//                name: "RoleGuid".to_string(),
//                tpe: "varchar".to_string(),
//            },
//            Field {
//                name: "TwoKey".to_string(),
//                tpe: "int".to_string(),
//            },
//            Field {
//                name: "CreateTime".to_string(),
//                tpe: "varchar".to_string(),
//            },
//            Field {
//                name: "CreateDatetime".to_string(),
//                tpe: "date".to_string(),
//            },
//            Field {
//                name: "CreateDate".to_string(),
//                tpe: "datetime".to_string(),
//            },
//            Field {
//                name: "CreateTimestamp".to_string(),
//                tpe: "int".to_string(),
//            },
//        ];
//        let table = Table::default(db, model, pks, fields);
//
//        let dbr = DBRoute {
//            engine: String::from("Mysql"),
//            user: String::from("snlan"),
//            pass: String::from("snlan"),
//            host: String::from("www.snlan.top"),
//            port: 3306,
//            name: String::from("block"),
//        };
//        let db = open_db(&dbr).unwrap();
//        (table, mem, db)
//    }
//
//    #[test]
//    fn test_mem_del() {
//        let (table, mut mem, _) = get_table_conn();
//        let res = mem
//            .del(
//                Arc::new(table),
//                vec!["block:TbTestModel:RoleGuid,TwoKey:0000009b790008004b64fb,3".to_string()],
//            )
//            .unwrap();
//
//        assert_eq!(res, 1);
//    }
//
//    #[test]
//    fn test_mem_load_update() {
//        let (table, mut mem, db) = get_table_conn();
//        let data = r##"{"conditions":{"RoleGuid__eq":"0000009b790008004b64fb","TwoKey__eq":"3","operator":"AND"},"values":{"CreateDate":"2017-00-00","CreateDatetime":"2017-00-00 09:16:55","CreateTime":"10:00:00","CreateTimestamp":"1"}}"##;
//        let body: Value = serde_json::from_str(data).unwrap();
//
//        let res = mem
//            .load_update(Arc::new(table), body, Arc::new(Mutex::new(db)))
//            .unwrap();
//        assert_eq!(res.len(), 1);
//    }
//
//    #[test]
//    fn test_mem_load_find() {
//        let (table, mut mem, db) = get_table_conn();
//        let data = r##"{"RoleGuid__eq":"0000009b790008004b64fb","TwoKey__eq":3,"operator":"AND"}"##;
//        let conditions: Value = serde_json::from_str(data).unwrap();
//
//        let cond = conditions.clone();
//        let cond = cond.as_object().unwrap();
//        let table = Arc::new(table);
//        let (pv_map, _) = MemContext::match_pk(table.clone(), cond);
//        let fields = vec![
//            Field {
//                name: "RoleGuid".to_string(),
//                tpe: "varchar".to_string(),
//            },
//            Field {
//                name: "TwoKey".to_string(),
//                tpe: "int".to_string(),
//            },
//        ];
//        let res = mem
//            .load_find(table, pv_map, conditions, Arc::new(Mutex::new(db)), &fields)
//            .unwrap();
//        println!("{}", res);
//    }
//}
