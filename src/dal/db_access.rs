use std::sync::{Arc, Mutex};
use mysql::Value;
use mysql::Error as MyError;
use serde_json::Value as Jvalue;
use dal::db::DB;
use dal::table::Table;
use dal::error::Error as DalError;
use std::rc::Rc;
use mysql::PooledConn;

#[derive(Copy, Clone)]
pub enum Dml {
    Insert,
    Delete,
    Update,
    Select,
}

pub struct Access {
    tbl: Rc<Table>,
    sql: String,
    params: Vec<(String, Value)>,
    dml: Dml,
    body: Jvalue,
}

impl Access {
    pub fn new(tbl: Rc<Table>, dml: Dml, body: Jvalue) -> Access {
        Access {
            tbl,
            sql: String::new(),
            params: Vec::new(),
            dml,
            body,
        }
    }

    pub fn exec_sql(&mut self, db: Arc<Mutex<DB>>) -> Result<Jvalue, DalError> {
        self.build();
        let mut db = db.lock()
            .map_err(|e| DalError::CommonError { info: format!("get db lock error: {:?}", e) })?;
        let mut conn = db.get_conn()?;
        let qr = conn.prep_exec(self.sql.clone(), self.params.clone())?;

        match self.dml {
            Dml::Select => {
                let ts = qr.map(|x| x.unwrap())
                    .map(|row| {
                        println!("--{:#?}", row);
                        row
                    })
                    .collect::<Vec<_>>();
                Ok(Jvalue::from(1))
            },
            _ => Ok(Jvalue::from(qr.affected_rows() as f64)),
        }
    }

    fn build(&mut self) -> Result<(), DalError> {
        match self.dml {
            Dml::Insert => self.insert(),
            Dml::Delete => self.delete(),
            Dml::Update => self.update(),
            Dml::Select => self.select(),
        }
    }

    fn insert(&mut self) -> Result<(), DalError> {
        let statement = "INSERT INTO %s(%s) VALUES(%s)";

        let values = self.body.get("values").ok_or(DalError::CommonError { info: "invalid json format".to_string() })?;
        let fvMap = values.as_object().ok_or(DalError::CommonError { info: "invalid json format at token 'values'".to_string() })?;


        let mut f_list: Vec<String> = Vec::new();
        for (f, v) in fvMap {
            let dbv = match v.clone() {
                Jvalue::String(s) => Value::from(s),
                Jvalue::Number(n) => {
                    if n.is_f64() {
                        Value::from(n.as_f64().unwrap())
                    } else if n.is_i64() {
                        Value::from(n.as_i64().unwrap())
                    } else if n.is_u64() {
                        Value::from(n.as_u64().unwrap())
                    } else {
                        continue;
                    }
                }
                _ => continue,
            };
            self.params.push((f.clone(), dbv));
            f_list.push(f.clone());
        }
        let v_list = f_list.iter()
            .map(|f| format!(":{}", f.clone()))
            .collect::<Vec<_>>();
        self.sql = format!("INSERT INTO {}({}) VALUES({})", self.tbl.get_model(), f_list.join(", "), v_list.join(", "));
        println!("SQL->{}", self.sql);
        Ok(())
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

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use dal::error::Error as DalError;
    use dal::db_access::Access;
    use dal::table::Table;
    use dal::table::Field;
    use dal::db_access::Dml;
    use std::rc::Rc;
    use dal::db::DB;
    use config::DBRoute;
    use dal::db::open_db;
    use std::sync::Arc;
    use std::sync::Mutex;

    fn new(dml: Dml, body: Value) -> Access {
        let db = "block".to_string();
        let model = "payment".to_string();
        let pks = vec!["customer_id".to_string()];
        let fields = vec![Field { name: "customer_id".to_string(), tpe: "int".to_string() },
                          Field { name: "amount".to_string(), tpe: "int".to_string() },
                          Field { name: "account_name".to_string(), tpe: "text".to_string() },
        ];
        let table = Table::default(db, model, pks, fields);

        Access::new(Rc::new(table), dml, body)
    }


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
    fn test_access_insert() {
        let data = r##"{"values":{"customer_id":4,"amount":456,"account_name":"lucy"}}"##;
        let v: Value = serde_json::from_str(data).unwrap();
        let mut access = new(Dml::Insert, v);

        let db = Arc::new(Mutex::new(get_db()));
        let exec_res = access.exec_sql(db).unwrap();

        println!("{}", exec_res);
        panic!("F")
    }

    #[test]
    fn test_access_select() {

    }
}