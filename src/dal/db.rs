use std::collections::HashMap;
use mysql::{PooledConn, Error, Pool, Value};
use config::DBRoute;
use dal::table::{Table, TableSchema};

#[allow(dead_code)]
pub struct DB {
    db_name: String,
    pool: Pool,
    pub tables: HashMap<String, Table>,
    counter: u64,
}

#[allow(dead_code)]
impl DB {
    pub fn new(name: String, pool: Pool) -> DB {
        DB {
            db_name: name,
            pool,
            tables: HashMap::new(),
            counter: 0,
        }
    }

    pub fn load_db(&mut self) -> Result<(), Error> {
        let sql = "SELECT TABLE_NAME \
        FROM information_schema.TABLES \
        WHERE TABLE_SCHEMA = :table_schema";
        let mut params: Vec<(String, Value)> = Vec::new();
        params.push((String::from("table_schema"), Value::from(self.db_name.clone())));
        let mut conn = self.pool.get_conn()?;
        let ts: Vec<String> = conn.prep_exec(sql, params)
            .map(|result| {
                result.map(|x| x.unwrap()).map(|row| {
                    let table_name = mysql::from_row(row);
                    table_name
                }).collect::<Vec<_>>()
            })?;

        println!("{:?}", ts);
        for t in ts {
            self.load_table(t)?
        }
        Ok(())
    }

    pub fn load_table(&mut self, table_name: String) -> Result<(), Error> {
        let sql = "SELECT TABLE_NAME, COLUMN_NAME, DATA_TYPE, COLUMN_KEY
        FROM information_schema.COLUMNS
        WHERE TABLE_SCHEMA = :table_schema
        AND TABLE_NAME = :table_name";
        let mut params: Vec<(String, Value)> = Vec::new();
        params.push((String::from("table_schema"), Value::from(self.db_name.clone())));
        params.push((String::from("table_name"), Value::from(table_name.clone())));
        let mut conn = self.pool.get_conn()?;
        let fields = conn.prep_exec(sql, params)
            .map(|result| {
                result.map(|x| x.unwrap()).map(|row| {
                    let (table_name, column_name, data_type, column_key) = mysql::from_row(row);
                    TableSchema {
                        table_name,
                        column_name,
                        data_type,
                        column_key,
                    }
                }).collect::<Vec<_>>()
            })?;

        let table = Table::new(self.db_name.clone(), table_name.clone(), fields);
        self.tables.insert(table_name, table);
        Ok(())
    }

    pub fn get_conn(&mut self) -> Result<PooledConn, Error> {
        self.counter += 1;
        self.pool.get_conn()
    }

    pub fn release_conn(&mut self) {
        self.counter -= 1;
    }

    pub fn close(&self) {
        use std::mem::drop;
        drop(self);
    }
}

pub fn open_db(cfg: DBRoute) -> Result<DB, Error> {
    let addr = format!("mysql://{}:{}@{}/{}", cfg.user, cfg.pass, cfg.addr, cfg.db);
    match Pool::new(addr) {
        Ok(pool) => Ok(DB::new(cfg.db, pool)),
        Err(err) => Err(err),
    }
}

pub struct DBManger {
    dbs: HashMap<String, DB>,
}

impl DBManger {
    pub fn new() -> DBManger {
        DBManger { dbs: HashMap::new() }
    }

    pub fn add_db(&mut self, db: DB) {
        let name = db.db_name.clone();
        self.dbs.insert(name, db);
    }

    pub fn close_db(&mut self, name: String) {
        let db = self.dbs.get(&name).unwrap();
        db.close();
    }

    pub fn get_db(&self, db: String) -> Option<Box<&DB>> {
        match self.dbs.get(&db) {
            Some(db) => Some(Box::new(db)),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_db()->DB {

        let dbr = DBRoute {
            engine: String::from("Mysql"),
            user: String::from("snlan"),
            pass: String::from("snlan"),
            addr: String::from("www.snlan.top"),
            db: String::from("block"),
        };
        open_db(dbr).unwrap()

    }

    #[test]
    fn test_conn() {
        let mut db = get_db();
        let res = db.load_db().unwrap();

        println!("{:#?}", db.tables);
    }

    #[test]
    fn test_open_db() {
        let mut db = get_db();
        let mut con = db.get_conn().unwrap();
//        con.prep_exec()
    }
}