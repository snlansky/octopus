use dal::dao::Dao;
use std::sync::{Arc, Mutex};
use serde_json::Value as JsValue;
use dal::db::DB;
use dal::table::Table;
use dal::error::Error;
use std::rc::Rc;
use dal::dao::DML;
use dal::dao::DaoResult;
use dal::mem::Mem;

pub struct Support {

}

pub fn add(db: Arc<Mutex<DB>>, tbl: Rc<Table>, body: JsValue) -> Result<JsValue, Error> {
    let mut dao = Dao::new(tbl, DML::Insert, body);
    match  dao.exec_sql(db)? {
        DaoResult::Affected(i)=>Ok(json!(i)),
        _ => panic!("program bug"),
    }
}

pub fn remove(db: Arc<Mutex<DB>>, mem:Option<Mem>, tbl: Table, body: JsValue) -> Result<JsValue, Error> {
    let rc_tbl = Rc::new(tbl);
    if let Some(mut mem) = mem {
        let mut dao = Dao::new(rc_tbl.clone(), DML::Select, body.clone());
        let rows = match dao.exec_sql(db.clone())? {
            DaoResult::Rows(rows) => rows,
            _ => panic!("program bug"),
        };
        let mids = rows.iter()
            .map(|row|{
                rc_tbl.clone().get_model_key(row)
            })
            .collect::<Vec<_>>();
        if mids.len() > 0 {
            mem.del(rc_tbl.clone(), mids)?;
        }
    }

    let mut dao = Dao::new(rc_tbl.clone(), DML::Delete, body);
    match  dao.exec_sql(db.clone())? {
        DaoResult::Affected(i)=>Ok(json!(i)),
        _ => panic!("program bug"),
    }
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
    use dal::support::modify;
    use config::config::MemRoute;
    use dal::table::Table;
    use dal::mem::Mem;
    use dal::db::DB;
    use dal::mem::open_client;
    use dal::table::Field;
    use config::config::DBRoute;
    use dal::db::open_db;
    use std::sync::Arc;
    use std::sync::Mutex;
    use dal::support::remove;
    use serde_json::Value;
    use std::rc::Rc;

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
            addr: String::from("www.snlan.top"),
            db: String::from("block"),
        };
        let db = open_db(dbr).unwrap();
        (table, mem, db)
    }

    #[test]
    fn test_remove() {
        let (table, mut mem, mut db) = get_table_conn();
        let data = r##"{"RoleGuid__eq":"0000009b790008004b64fb","TwoKey__eq":3,"operator":"AND"}"##;
        let body: Value = serde_json::from_str(data).unwrap();
        let mem: Option<Mem> = Some(mem);
        let i = remove(Arc::new(Mutex::new(db)), mem, table, body).unwrap();
        assert_eq!(json!(1), i);
    }
}