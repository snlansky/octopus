use std::sync::{Arc, Mutex};
use mysql::Value as MyValue;
use serde_json::Value as JsValue;
use dal::db::DB;
use dal::table::Table;
use dal::error::Error;
use std::rc::Rc;

pub enum DML {
    Insert,
    Delete,
    Update,
    Select,
}

pub struct Access {
    tbl: Rc<Table>,
    sql: String,
    params: Vec<(String, MyValue)>,
    dml: DML,
    body: JsValue,
}

impl Access {
    pub fn new(tbl: Rc<Table>, dml: DML, body: JsValue) -> Access {
        Access {
            tbl,
            sql: String::new(),
            params: Vec::new(),
            dml,
            body,
        }
    }

    pub fn exec_sql(&mut self, db: Arc<Mutex<DB>>) -> Result<JsValue, Error> {
        self.build()?;
        let mut db = db.lock()
            .map_err(|e| Error::CommonError { info: format!("get db lock error: {:?}", e) })?;
        let mut conn = db.get_conn()?;
        let qr = conn.prep_exec(self.sql.clone(), self.params.clone())?;

        match self.dml {
            DML::Select => {
                let ts = qr.map(|x| x.unwrap())
                    .map(|row| {
                        println!("--{:#?}", row);
                        row
                    })
                    .collect::<Vec<_>>();
                Ok(JsValue::from(1))
            }
            _ => Ok(JsValue::from(qr.affected_rows() as f64)),
        }
    }

    fn build(&mut self) -> Result<(), Error> {
        match self.dml {
            DML::Insert => self.insert(),
            DML::Delete => self.delete(),
            DML::Update => self.update(),
            DML::Select => self.select(),
        }
    }

    fn insert(&mut self) -> Result<(), Error> {
        let values = self.body.get("values").ok_or(Error::CommonError { info: "invalid json format".to_string() })?;
        let fv_map = values.as_object().ok_or(Error::CommonError { info: "invalid json format at token 'values'".to_string() })?;

        let mut f_list: Vec<String> = Vec::new();
        let mut v_list: Vec<String> = Vec::new();
        for (f, v) in fv_map {
            let dbv = match v.clone() {
                JsValue::String(s) => MyValue::from(s),
                JsValue::Number(n) => {
                    if n.is_f64() {
                        MyValue::from(n.as_f64().unwrap())
                    } else if n.is_i64() {
                        MyValue::from(n.as_i64().unwrap())
                    } else if n.is_u64() {
                        MyValue::from(n.as_u64().unwrap())
                    } else {
                        continue;
                    }
                },
                _ => continue,
            };
            f_list.push(format!("`{}`",f.clone()));
            v_list.push(format!(":{}", f.to_lowercase()));
            self.params.push((f.to_lowercase(), dbv));
        }
        self.sql = format!("INSERT INTO `{}` ({}) VALUES({})", self.tbl.get_model(), f_list.join(","), v_list.join(", "));
        println!("SQL->{}", self.sql);
        Ok(())
    }
    fn delete(&mut self) -> Result<(), Error> {
        let statement = "DELETE FROM %s WHERE %s";
        unimplemented!()
    }
    fn update(&mut self) -> Result<(), Error> {
        let statement = "UPDATE %s SET %s WHERE %s";
        unimplemented!()
    }
    fn select(&mut self) -> Result<(), Error> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use dal::error::Error as Error;
    use dal::db_access::Access;
    use dal::table::Table;
    use dal::table::Field;
    use dal::db_access::DML;
    use std::rc::Rc;
    use dal::db::DB;
    use config::DBRoute;
    use dal::db::open_db;
    use std::sync::Arc;
    use std::sync::Mutex;

    fn new(dml: DML, body: Value) -> Access {
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

        Access::new(Rc::new(table), dml, body)
    }


    fn get_db() -> DB {
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
        let data = r##"{"values":{"RoleGuid":"0000009b790008004b65fb","TwoKey":2,"CreateTime":"22:00:00","CreateDatetime":"2019-02-08","CreateDate":"2019-02-06 01:24:38","CreateTimestamp":1480580000}}"##;

        let v: Value = serde_json::from_str(data).unwrap();
        let mut access = new(DML::Insert, v);

        let db = Arc::new(Mutex::new(get_db()));
        let exec_res = access.exec_sql(db).unwrap();

        println!("{}", exec_res);
        panic!("F")
    }

    #[test]
    fn test_access_update() {
        let data = r##"{"conditions":{"TwoKey__gte":9,"TwoKey__lte":1,"operator":"OR"},"values":{"CreateDate":"2017-02-23","CreateTimestamp":456}}"##;
        let v: Value = serde_json::from_str(data).unwrap();
        let mut access = new(DML::Update, v);

        let db = Arc::new(Mutex::new(get_db()));
        let exec_res = access.exec_sql(db).unwrap();

        println!("{}", exec_res);
        panic!("F")
    }
}