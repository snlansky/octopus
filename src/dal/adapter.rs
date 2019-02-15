use dal::dao::Dao;
use std::sync::{Arc, Mutex};
use serde_json::Value as JsValue;
use dal::db::DB;
use dal::table::Table;
use dal::error::Error;
use std::rc::Rc;
use dal::dao::DML;
use dal::dao::DaoResult;

pub fn add(db: Arc<Mutex<DB>>, tbl: Rc<Table>, body: JsValue) -> Result<JsValue, Error> {
    let mut dao = Dao::new(tbl, DML::Insert, body);
    dao.exec_sql(db)
}

pub fn remove(db: Arc<Mutex<DB>>, tbl: Rc<Table>, body: JsValue) -> Result<JsValue, Error> {
    let mut dao = Dao::new(tbl, DML::Delete, body);
    dao.exec_sql(db)
}

pub fn modify() -> Result<JsValue, Error> {
    let d1 = DaoResult::Affected(12);
    let i= match d1 {
        DaoResult::Affected(t) => t,
        _ => panic!("program bug")
    };
    Ok(json!(i))
}

#[cfg(test)]
mod tests {
    use dal::adapter::modify;

    #[test]
    fn test_modify() {
        let i = modify().unwrap();
        println!("{}", i);
    }
}