use dal::dao::Dao;
use std::sync::{Arc, Mutex};
use serde_json::Value as JsValue;
use dal::db::DB;
use dal::table::Table;
use dal::dao::DML;
use dal::dao::DaoResult;
use dal::mem::MemContext;
use threadpool::ThreadPool;
use serde_json::Map;
use dal::value::ConvertTo;
use discovery::Register;
use config::Provider;
use config::Services;
use std::collections::HashMap;
use dal::Error;
use dal::Error::CommonError;
use dal::mem::Mem;
use dal::interface::Close;
use dal::Route;


pub struct Support<T: Provider> {
    register: Arc<Register>,
    provider: T,
    port:i32,
    routes: HashMap<String, Route>,
}

impl<T: Provider> Support<T> {
    pub fn new(register: Arc<Register>, mut provider: T, pool: &ThreadPool) -> Self {
        let services = provider.watch();
        info!("\n{:?}", services);
        let mut support = Support {
            register,
            provider,
            port:services.port,
            routes: HashMap::new(),
        };
        support.start(pool);
        support
    }

    pub fn port(&self) -> i32 {
        self.port
    }

    pub fn data_route(&self, db_alias: &String) -> Option<&Route> {
        self.routes.get(db_alias)
            .map(|f| {
                f
            })
    }

    pub fn start(&mut self, pool: &ThreadPool) {
        loop {
            let services = self.provider.watch();
            info!("\n{:?}", services);
            self.update(&services);
        }
    }

    fn update(&mut self, services:&Services) {}

//    fn update(&mut self, services: &Services) {
//        if self.services.data == services.data {
//            return;
//        }
//
//        // 先更新没有的配置
//        for (alias, route) in &services.data {
//            match self.services.data.get_mut(alias) {
//                Some(old) => {
//                    if let Some(r) = self.routes.get_mut(alias) {
//                        if old.db != route.db {
//                            Self::update_db_client(&mut r.0, &route.db)
//                                .map(|_| old.db = route.db)
//                                .map_err(|err| error!("update db client {:?} failed, reason: {:?}", &route.db, err));
//                        }
//                        if old.mem != route.mem {
//                            Self::update_mem_client(&mut r.1, &route.mem)
//                                .map(|_| old.mem = route.mem)
//                                .map_err(|err| error!("update mem client {:?} failed, reason: {:?}", &route.mem, err));
//                        }
//                    } else {
//                        unreachable!()
//                    }
//                }
//                None => {
//                    match Self::get_route(&route) {
//                        Ok(r) => {
//                            self.routes.insert(alias.clone(), r);
//                            self.services.data.insert(alias.clone(), route.clone());
//                        }
//                        Err(err) => {
//                            error!("update client {:?} failed, reason: {:?}", &route, err);
//                        }
//                    }
//                }
//            }
//        }
//
//        // 删除old配置
//        let mut del = Vec::new();
//        for (alias, route) in &self.services.data {
//            if !services.data.contains_key(alias) {
//                del.push(alias)
//            }
//        }
//        for alias in del {
//            self.services.data.remove(alias);
//            if let Some((db, mem)) = self.routes.remove(alias) {
//                db.lock()
//                    .map(|f| f.close())
//                    .map_err(|e| error!("get db<{}> lock failed when close db, reason: {:?}", alias, e));
//                if let Some(m) = mem {
//                    m.close();
//                }
//            }
//        }
//    }
//
//    fn get_route(route: &DataRoute) -> Result<(Arc<Mutex<DB>>, Option<Arc<Mutex<Mem>>>), Error> {
//        let db = ArcMutex(open_db(&route.db)?);
//        match &route.mem {
//            Some(mr) => {
//                let mem = ArcMutex(Mem::new(open_client(mr)?));
//                Ok((db, Some(mem)))
//            }
//            None => Ok((db, None)),
//        }
//    }
//
//    fn update_db_client(r: &mut Arc<Mutex<DB>>, route: &DBRoute) -> Result<(), Error> {
//        let db = open_db(route)?;
//        let locked = r.lock()
//            .map_err(|e| Error::CommonError { info: format!("get db lock error: {:?}", e) })?;
//        locked.close();
//        *r = ArcMutex(db);
//        Ok(())
//    }
//
//    fn update_mem_client(r: &mut Option<Arc<Mutex<Mem>>>, route: &Option<MemRoute>) -> Result<(), Error> {
//        if let Some(mr) = route {
//            let conn = open_client(mr)?;
//            r.close();
//            *r = Some(ArcMutex(Mem::new(conn)));
//        } else {
//            r.close();
//            *r = None;
//        }
//        Ok(())
//    }
}


pub fn add(db: Arc<Mutex<DB>>, tbl: Arc<Table>, body: JsValue) -> Result<JsValue, Error> {
    let mut dao = Dao::new(tbl, DML::Insert, body);
    match dao.exec_sql(db)? {
        DaoResult::Affected(i) => Ok(json!(i)),
        _ => unreachable!(),
    }
}

pub fn remove(db: Arc<Mutex<DB>>, mem: Option<MemContext>, tbl: Arc<Table>, body: JsValue) -> Result<JsValue, Error> {
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

impl Close for Option<Arc<Mutex<Mem>>> {
    fn close(&self) {
        if let Some(m) = self {
            m.close();
        }
    }
}

impl Close for Arc<Mutex<Mem>> {
    fn close(&self) {
        self.lock()
            .map(|_f| {}) // TODO 通过作用域进行释放
            .map_err(|e| error!("get mem lock failed, reason: {:?}", e));
    }
}

pub fn modify(pool: &ThreadPool, db: Arc<Mutex<DB>>, mem: Option<MemContext>, table: Arc<Table>, body: JsValue) -> Result<JsValue, Error> {
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

pub fn find(db: Arc<Mutex<DB>>, mem: Option<MemContext>, table: Arc<Table>, body: JsValue) -> Result<JsValue, Error> {
    let cond = body.as_object()
        .ok_or(CommonError { info: "invalid json format".to_string() })?.clone();
    let (pv_map, match_pk) = MemContext::match_pk(table.clone(), &cond);

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
    use dal::mem::MemContext;
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
    use dal::mem::Mem;


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