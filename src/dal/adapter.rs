use dal::dao::Dao;
use std::sync::{Arc, Mutex};
use serde_json::Value as JsValue;
use dal::db::DB;
use dal::table::Table;
use dal::error::Error;
use std::rc::Rc;
use dal::dao::DML;

pub fn add(db:Arc<Mutex<DB>>, tbl: Rc<Table>, body: JsValue) -> Result<JsValue, Error> {
    let mut dao = Dao::new(tbl, DML::Insert, body);
    dao.exec_sql(db)
}

pub fn remove(db:Arc<Mutex<DB>>, tbl: Rc<Table>, body: JsValue) -> Result<JsValue, Error> {
    let mut dao = Dao::new(tbl, DML::Delete, body);
    dao.exec_sql(db)
}