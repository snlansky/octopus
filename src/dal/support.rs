use dal::dao::Dao;
use std::sync::{Arc, Mutex};
use serde_json::Value as JsValue;
use dal::db::DB;
use dal::table::Table;
use dal::error::Error;
use dal::dao::DML;
use dal::dao::DaoResult;
use dal::mem::Mem;
use threadpool::ThreadPool;
use serde_json::Map;
use dal::value::ConvertTo;
use discovery::Register;
use config::Provider;

pub struct Support<T: Provider> {
    register: Arc<Register>,
    provider: T,
}

impl<T: Provider> Support<T> {
    pub fn new(register: Arc<Register>, mut provider:T) -> Self {

//        loop {
            let s = provider.watch();
            println!("-->{:?}", s);
//        }
        Support { register, provider }
    }
}


pub fn add(db: Arc<Mutex<DB>>, tbl: Arc<Table>, body: JsValue) -> Result<JsValue, Error> {
    let mut dao = Dao::new(tbl, DML::Insert, body);
    match dao.exec_sql(db)? {
        DaoResult::Affected(i) => Ok(json!(i)),
        _ => unreachable!(),
    }
}

pub fn remove(db: Arc<Mutex<DB>>, mem: Option<Mem>, tbl: Arc<Table>, body: JsValue) -> Result<JsValue, Error> {
    if let Some(mut mem) = mem {
        let mut dao = Dao::new(tbl.clone(), DML::Select, body.clone());
        let rows = match dao.exec_sql(db.clone())? {
            DaoResult::Rows(rows) => rows,
            _ => unreachable!(),
        };
        let mids = rows.iter()
            .map(|row| {
                tbl.get_model_key(row)
            })
            .collect::<Vec<_>>();
        if mids.len() > 0 {
            mem.del(tbl.clone(), mids)?;
        }
    }

    let mut dao = Dao::new(tbl, DML::Delete, body);
    match dao.exec_sql(db.clone())? {
        DaoResult::Affected(i) => Ok(json!(i)),
        _ => unreachable!(),
    }
}

pub fn modify(pool: &ThreadPool, db: Arc<Mutex<DB>>, mem: Option<Mem>, table: Arc<Table>, body: JsValue) -> Result<JsValue, Error> {
    let db1 = db.clone();
    let body1 = body.clone();
    let up_dao = move |tbl: Arc<Table>| -> Result<JsValue, Error>{
        let mut dao = Dao::new(tbl, DML::Update, body1);
        match dao.exec_sql(db1)? {
            DaoResult::Affected(i) => Ok(json!(i)),
            _ => unreachable!(),
        }
    };

    if let Some(mut mem) = mem {
        let mids = mem.load_update(table.clone(), body.clone(), db.clone())?;
        let i = mids.len();
        let t1 = table.clone();
        pool.execute(move || {
            let res = up_dao(t1.clone());
            if res.is_err() {
                mem.del(t1, mids).unwrap();
            }
        });
        Ok(json!(i))
    } else {
        up_dao(table)
    }
}

pub fn find(db: Arc<Mutex<DB>>, mem: Option<Mem>, table: Arc<Table>, body: JsValue) -> Result<JsValue, Error> {
    let cond = body.as_object()
        .ok_or(Error::CommonError { info: "invalid json format".to_string() })?.clone();
    let (pv_map, match_pk) = Mem::match_pk(table.clone(), &cond);

    if mem.is_some() && match_pk {
        let mut mem = mem.unwrap();
        mem.load_find(table.clone(), pv_map, body.clone(), db.clone(), table.get_fields())
    } else {
        let mut dao = Dao::new(table.clone(), DML::Select, body);
        match dao.exec_sql(db.clone())? {
            DaoResult::Rows(rows) => {
                let rows = rows.iter()
                    .map(|f| {
                        let v: Map<String, JsValue> = f.convert();
                        JsValue::Object(v)
                    })
                    .collect::<Vec<JsValue>>();
                Ok(JsValue::Array(rows))
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use dal::support::modify;
    use config::MemRoute;
    use dal::table::Table;
    use dal::mem::Mem;
    use dal::db::DB;
    use dal::mem::open_client;
    use dal::table::Field;
    use config::DBRoute;
    use dal::db::open_db;
    use std::sync::Arc;
    use std::sync::Mutex;
    use dal::support::remove;
    use serde_json::Value;
    use threadpool::ThreadPool;
    use std::{thread, time};
    use dal::support::find;


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
            host: String::from("www.snlan.top"),
            port: 3306,
            name: String::from("block"),
        };
        let db = open_db(dbr).unwrap();
        (table, mem, db)
    }

    #[test]
    fn test_remove() {
        let (table, mem, db) = get_table_conn();
        let data = r##"{"RoleGuid__eq":"0000009b790008004b64fb","TwoKey__eq":3,"operator":"AND"}"##;
        let body: Value = serde_json::from_str(data).unwrap();
        let mem: Option<Mem> = Some(mem);
        let i = remove(Arc::new(Mutex::new(db)), mem, Arc::new(table), body).unwrap();
        assert_eq!(json!(1), i);
    }

    #[test]
    fn test_modify() {
        let (table, mem, db) = get_table_conn();
        let data = r##"{"conditions":{"TwoKey__gte":1,"TwoKey__lte":9, "TwoKey__in":[21,31],"operator":"OR", "RoleGuid__like":"%9b%"},"values":{"CreateDate":"2017-02-23","CreateTimestamp":123}}"##;
        let body: Value = serde_json::from_str(data).unwrap();

        let pool = ThreadPool::new(2);
        let i = modify(&pool, Arc::new(Mutex::new(db)), Some(mem), Arc::new(table), body).unwrap();
        thread::sleep(time::Duration::from_secs(2));
        assert_eq!(json!(2), i);
    }

    #[test]
    fn test_find() {
        let (table, mem, db) = get_table_conn();
        let data = r##"{"TwoKey__eq":2,"RoleGuid__eq":"0000009b120008004b64fb","limit":3,"operator":"AND","order":"TwoKey__DESC,CreateTimestamp__ASC"}"##;
        let body: Value = serde_json::from_str(data).unwrap();
        let res = find(Arc::new(Mutex::new(db)), Some(mem), Arc::new(table), body).unwrap();

        println!("{}", res);
    }
}