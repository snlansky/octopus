use config::Provider;
use config::Services;
use dal::dao::Dao;
use dal::dao::DaoResult;
use dal::dao::DML;
use dal::db::DB;
use dal::mem::MemContext;
use dal::table::Table;
use dal::utils::arc_mutex;
use dal::value::ConvertTo;
use dal::Error;
use dal::Error::CommonError;
use dal::Route;
use discovery::Register;
use serde_json::Map;
use serde_json::Value as JsValue;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

pub struct Support {
    register: Arc<Register>,
    port: i32,
    routes: HashMap<String, Route>,
}

impl Support {
    pub fn new(
        register: Arc<Register>,
        mut provider: Provider,
        pool: &ThreadPool,
    ) -> Arc<Mutex<Self>> {
        let services = provider.watch();
        info!("{:?}", services);
        let mut support = Support {
            register,
            port: services.port,
            routes: HashMap::new(),
        };
        support.update(&services);
        let support = arc_mutex(support);
        async_update(support.clone(), provider, pool);
        support
    }

    pub fn port(&self) -> i32 {
        self.port
    }

    pub fn data_route(&self, db_alias: &str) -> Option<&Route> {
        self.routes.get(db_alias)
    }

    fn update(&mut self, services: &Services) {
        for (alias, data) in &services.data {
            if let Some(route) = self.routes.get_mut(alias) {
                // 有就更新
                if !route.eq(data) {
                    if let Err(e) = route.update(data) {
                        error!(
                            "update {} failed, reason {:?}, config: {:?}",
                            alias, e, &data
                        );
                    };
                }
            }
            if !self.routes.contains_key(alias) {
                let route = Route::new(alias, data);
                match route {
                    Ok(r) => {
                        self.routes.insert(alias.clone(), r);
                    }
                    Err(e) => error!("save {} failed, reason {:?}, config: {:?}", alias, e, &data),
                };
            }
        }

        let list = self
            .routes
            .iter()
            .map(|(k, _)| k.clone())
            .filter(|k| !services.data.contains_key(k))
            .collect::<Vec<_>>();
        for alias in list {
            info!("remove {}", alias);
            self.routes.remove(&alias);
        }
    }
}

fn async_update(s: Arc<Mutex<Support>>, mut provider: Provider, pool: &ThreadPool) {
    pool.execute(move || loop {
        let services = provider.watch();
        let mut p = s.lock().unwrap();
        info!("\n{:?}", services);
        p.update(&services);
    });
}

pub fn add(db: Arc<Mutex<DB>>, tbl: Arc<Table>, body: JsValue) -> Result<JsValue, Error> {
    let mut dao = Dao::new(tbl, DML::Insert, body);
    match dao.exec_sql(db)? {
        DaoResult::Affected(i) => Ok(json!(i)),
        _ => unreachable!(),
    }
}

pub fn remove(
    db: Arc<Mutex<DB>>,
    mem: Option<MemContext>,
    tbl: Arc<Table>,
    body: JsValue,
) -> Result<JsValue, Error> {
    if let Some(mut mem) = mem {
        let mut dao = Dao::new(tbl.clone(), DML::Select, body.clone());
        let rows = match dao.exec_sql(db.clone())? {
            DaoResult::Rows(rows) => rows,
            _ => unreachable!(),
        };
        let mids = rows
            .iter()
            .map(|row| tbl.get_model_key(row))
            .collect::<Vec<_>>();
        if !mids.is_empty() {
            mem.del(tbl.clone(), mids)?;
        }
    }

    let mut dao = Dao::new(tbl, DML::Delete, body);
    match dao.exec_sql(db.clone())? {
        DaoResult::Affected(i) => Ok(json!(i)),
        _ => unreachable!(),
    }
}

pub fn modify(
    pool: &ThreadPool,
    db: Arc<Mutex<DB>>,
    mem: Option<MemContext>,
    table: Arc<Table>,
    body: JsValue,
) -> Result<JsValue, Error> {
    let db1 = db.clone();
    let body1 = body.clone();
    let up_dao = move |tbl: Arc<Table>| -> Result<JsValue, Error> {
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

pub fn find(
    db: Arc<Mutex<DB>>,
    mem: Option<MemContext>,
    table: Arc<Table>,
    body: JsValue,
) -> Result<JsValue, Error> {
    let cond = body
        .as_object()
        .ok_or(CommonError {
            info: "invalid json format".to_string(),
        })?
        .clone();
    let (pv_map, match_pk) = MemContext::match_pk(table.clone(), &cond);

    if mem.is_some() && match_pk {
        let mut mem = mem.unwrap();
        mem.load_find(
            table.clone(),
            pv_map,
            body.clone(),
            db.clone(),
            table.get_fields(),
        )
    } else {
        let mut dao = Dao::new(table.clone(), DML::Select, body);
        match dao.exec_sql(db.clone())? {
            DaoResult::Rows(rows) => {
                let rows = rows
                    .iter()
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
    use config::DBRoute;
    use config::MemRoute;
    use dal::db::open_db;
    use dal::db::DB;
    use dal::mem::open_client;
    use dal::mem::Mem;
    use dal::mem::MemContext;
    use dal::support::find;
    use dal::support::modify;
    use dal::support::remove;
    use dal::table::Field;
    use dal::table::Table;
    use serde_json::Value;
    use std::sync::Arc;
    use std::sync::Mutex;
    use std::{thread, time};
    use threadpool::ThreadPool;

    fn get_table_conn() -> (Table, MemContext, DB) {
        let r = MemRoute {
            host: "www.snlan.top".to_string(),
            port: 6379,
            pass: "snlan".to_string(),
            expire: 60 * 60,
            db: 0,
        };
        let conn = open_client(&r).unwrap();
        let mem = MemContext::new(Arc::new(Mutex::new(Mem::new(conn))));
        let db = "block".to_string();
        let model = "TbTestModel".to_string();
        let pks = vec!["RoleGuid".to_string(), "TwoKey".to_string()];
        let fields = vec![
            Field {
                name: "RoleGuid".to_string(),
                tpe: "varchar".to_string(),
            },
            Field {
                name: "TwoKey".to_string(),
                tpe: "int".to_string(),
            },
            Field {
                name: "CreateTime".to_string(),
                tpe: "varchar".to_string(),
            },
            Field {
                name: "CreateDatetime".to_string(),
                tpe: "date".to_string(),
            },
            Field {
                name: "CreateDate".to_string(),
                tpe: "datetime".to_string(),
            },
            Field {
                name: "CreateTimestamp".to_string(),
                tpe: "int".to_string(),
            },
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
        let db = open_db(&dbr).unwrap();
        (table, mem, db)
    }

    #[test]
    fn test_remove() {
        let (table, mem, db) = get_table_conn();
        let data = r##"{"RoleGuid__eq":"0000009b790008004b64fb","TwoKey__eq":3,"operator":"AND"}"##;
        let body: Value = serde_json::from_str(data).unwrap();
        let mem: Option<MemContext> = Some(mem);
        let i = remove(Arc::new(Mutex::new(db)), mem, Arc::new(table), body).unwrap();
        assert_eq!(json!(1), i);
    }

    #[test]
    fn test_modify() {
        let (table, mem, db) = get_table_conn();
        let data = r##"{"conditions":{"TwoKey__gte":1,"TwoKey__lte":9, "TwoKey__in":[21,31],"operator":"OR", "RoleGuid__like":"%9b%"},"values":{"CreateDate":"2017-02-23","CreateTimestamp":123}}"##;
        let body: Value = serde_json::from_str(data).unwrap();

        let pool = ThreadPool::new(2);
        let i = modify(
            &pool,
            Arc::new(Mutex::new(db)),
            Some(mem),
            Arc::new(table),
            body,
        )
        .unwrap();
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
