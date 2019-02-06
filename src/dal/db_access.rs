use std::sync::{Arc, Mutex};
use mysql::Value;
use mysql::Error as MyError;
use serde_json::Value as Jvalue;
use dal::db::DB;
use dal::table::Table;
use dal::error::Error as DalError;
use std::rc::Rc;

#[derive(Copy, Clone)]
pub enum Dml {
    Insert,
    Delete,
    Update,
    Select,
}

pub struct DBAccess {
    db: Arc<Mutex<DB>>,
    dml: Dml,
}

impl DBAccess {
    pub fn new(db: Arc<Mutex<DB>>, dml: Dml) -> DBAccess {
        DBAccess { db, dml }
    }

    pub fn exec_sql(&mut self, sql: String, args: Vec<String>) -> Result<u32, MyError> {
        let db = self.db.clone();
        let mut db = db.lock()?;
        let mut conn = db.get_conn()?;
        conn.prep_exec(sql, args);
        Ok(12)
    }
}

pub struct SQLBuilder {
    tbl: Rc<Table>,
    sql: String,
    params: Vec<(String, Value)>,
    dml: Dml,
    body: Jvalue,
}

impl SQLBuilder {
    pub fn new(tbl: Rc<Table>, dml: Dml, body: Jvalue) -> SQLBuilder {
        SQLBuilder {
            tbl,
            sql: String::new(),
            params: Vec::new(),
            dml,
            body,
        }
    }

    pub fn build(&mut self) -> Result<(), DalError> {
        match self.dml {
            Dml::Insert => self.insert(),
            Dml::Delete => self.delete(),
            Dml::Update => self.update(),
            Dml::Select => self.select(),
        }
    }

    fn insert(&mut self) -> Result<(), DalError> {

        let statement = "INSERT INTO %s(%s) VALUES(%s)";

        match &self.body {
            Jvalue::Object(obj) => {
                Ok(())
            }
            _ => Err(DalError::CommonError{info:"invalid json format".to_string()}),
        }
    }
    fn delete(&mut self) -> Result<(), DalError> {
        let statement = "DELETE FROM %s WHERE %s";
        unimplemented!()
    }
    fn update(&mut self) -> Result<(), DalError> {
        let statement = "UPDATE %s SET %s WHERE %s";
        unimplemented!()
    }
    fn select(&mut self) -> Result<(), DalError> {
        unimplemented!()
    }
}