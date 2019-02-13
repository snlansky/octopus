use std::sync::{Arc, Mutex};
use mysql::Value as MyValue;
use serde_json::Value as JsValue;
use dal::db::DB;
use dal::table::Table;
use dal::error::Error;
use std::rc::Rc;
use serde_json::Map;
use std::fmt::Display;
use mysql::Row;
use std::collections::HashMap;

pub enum DML {
    Insert,
    Delete,
    Update,
    Select,
}

pub struct Dao {
    tbl: Rc<Table>,
    sql: String,
    params: Vec<(String, MyValue)>,
    dml: DML,
    body: JsValue,
}

impl Dao {
    pub fn new(tbl: Rc<Table>, dml: DML, body: JsValue) -> Dao {
        Dao {
            tbl,
            sql: String::new(),
            params: Vec::new(),
            dml,
            body,
        }
    }

    pub fn exec_sql(&mut self, db: Arc<Mutex<DB>>) -> Result<JsValue, Error> {
        self.build()?;
        println!("SQL->{}", self.sql);
        let mut db = db.lock()
            .map_err(|e| Error::CommonError { info: format!("get db lock error: {:?}", e) })?;
        let mut conn = db.get_conn()?;
        let qr = conn.prep_exec(self.sql.clone(), self.params.clone())?;

        match self.dml {
            DML::Select => {
                let ts = qr.map(|x| x.unwrap())
                    .map(|row| {
                        Self::parse_row(row)
                    })
                    .map(|f|Self::map2js(f))
                    .collect::<Vec<_>>();
                Ok(JsValue::Array(ts))
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
        let fv_map = self.get_js("values")?;

        let mut f_list: Vec<String> = Vec::new();
        let mut v_list: Vec<String> = Vec::new();
        for (f, v) in fv_map {
            if let Some(dbv) = Self::js2my(&v) {
                f_list.push(format!("`{}`", f));
                v_list.push(format!(":{}", f.to_lowercase()));
                self.padd(f.to_lowercase(), dbv);
            }
        }
        self.sql = format!("INSERT INTO `{}` ({}) VALUES({})", self.tbl.get_model(), f_list.join(","), v_list.join(", "));
        Ok(())
    }

    fn delete(&mut self) -> Result<(), Error> {
        let mut cond_map = self.body.as_object().ok_or(Error::CommonError { info: "invalid json format".to_string() })?.clone();
        let opr = Self::extra_op(&mut cond_map)?;
        let cond = self.parse_op(&cond_map)?;
        if cond.len() > 0 {
            self.sql = format!("DELETE FROM {} WHERE {}", self.tbl.get_model(), cond.join(opr.as_str()));
        } else {
            self.sql = format!("DELETE FROM {}", self.tbl.get_model());
        }
        Ok(())
    }

    fn update(&mut self) -> Result<(), Error> {
        let fv_map = self.get_js("values")?;
        let mut cond_map = self.get_js("conditions")?;
        let opr = Self::extra_op(&mut cond_map)?;
        let cond = self.parse_op(&cond_map)?;

        let mut fmt_params = Vec::new();
        for (f, v) in fv_map {
            if let Some(dbv) = Self::js2my(&v) {
                fmt_params.push(format!("{} = :{}", f, f.to_lowercase()));
                self.padd(f.to_lowercase(), dbv);
            }
        }

        if cond.len() > 0 {
            self.sql = format!("UPDATE {} SET {} WHERE {}", self.tbl.get_model(), fmt_params.join(", "), cond.join(opr.as_str()));
        } else {
            self.sql = format!("UPDATE {} SET {}", self.tbl.get_model(), fmt_params.join(", "));
        }

        Ok(())
    }

    fn select(&mut self) -> Result<(), Error> {
        let mut where_clause: Vec<String> = Vec::new();

        let mut query = self.body.as_object().ok_or(Error::CommonError { info: "invalid json format".to_string() })?.clone();
        let opr = Self::extra_op(&mut query)?;
        if let Some(v) = query.remove("order") {
            let orders = v.as_str().ok_or(Error::CommonError { info: format!("invalid json format at token '{}'", "order") })?;
            where_clause.push(format!("ORDER BY {}", orders.replace("__", " ")));
        }
        if let Some(v) = query.remove("limit") {
            let limit = v.as_i64().ok_or(Error::CommonError { info: format!("invalid json format at token '{}'", "limit") })?;
            where_clause.push(format!("LIMIT {}", limit));
            if let Some(v) = query.remove("offset") {
                let offset = v.as_i64().ok_or(Error::CommonError { info: format!("invalid json format at token '{}'", "offset") })?;
                where_clause.push(format!("OFFSET {}", offset));
            }
        }
        let cond = self.parse_op(&query)?;
        let columns = self.tbl.get_fields().iter().map(|f| f.name.clone()).collect::<Vec<_>>();
        if cond.len() + where_clause.len() > 0 {
            self.sql = format!("SELECT {} FROM {} WHERE {} {}", columns.join(", "), self.tbl.get_model(), cond.join(opr.as_str()), where_clause.join(" "));
        } else {
            self.sql = format!("SELECT {} FROM {}", columns.join(", "), self.tbl.get_model());
        }
        Ok(())
    }

    fn padd(&mut self, field: String, value: MyValue) {
        self.params.push((field, value))
    }

    fn parse_op(&mut self, cond: &Map<String, JsValue>) -> Result<Vec<String>, Error> {
        let mut fmt_params: Vec<String> = Vec::new();

        let mut set_params = |f: String, opt: Option<MyValue>| -> Result<(), Error> {
            if let Some(_v) = opt {
                self.padd(f, _v);
                Ok(())
            } else {
                Err(Error::CommonError { info: "invalid json format".to_string() })
            }
        };
        let mut index = 0;
        for (f, v) in cond {
            let key: Vec<&str> = f.split("__").collect();
            if key.len() < 2 {
                continue;
            }
            let mut param = f.clone();
            let lower_f = format!("cond{}", index);
            let value = Self::js2my(v);
            match key[1] {
                "eq" => {
                    param = format!("{} = :{}", key[0], lower_f);
                    set_params(lower_f, value)?;
                    index += 1;
                }
                "ne" => {
                    param = format!("{} != :{}", key[0], lower_f);
                    set_params(lower_f, value)?;
                    index += 1;
                }
                "lt" => {
                    param = format!("{} < :{}", key[0], lower_f);
                    set_params(lower_f, value)?;
                    index += 1;
                }
                "lte" => {
                    param = format!("{} <= :{}", key[0], lower_f);
                    set_params(lower_f, value)?;
                    index += 1;
                }
                "gt" => {
                    param = format!("{} > :{}", key[0], lower_f);
                    set_params(lower_f, value)?;
                    index += 1;
                }
                "gte" => {
                    param = format!("{} >= :{}", key[0], lower_f);
                    set_params(lower_f, value)?;
                    index += 1;
                }
                "is" => {
                    param = format!("{} IS NULL", key[0]);
                }
                "isnot" => {
                    param = format!("{} IS NOT NULL", key[0]);
                }
                "in" => {
                    if !v.is_array() {
                        return Err(Error::CommonError { info: format!("invalid format at {}, it`s must json array", f) });
                    }
                    let list = v.as_array().unwrap().iter()
                        .map(|f| Self::js2my(f))
                        .filter(|f| f.is_some())
                        .map(|f| f.unwrap())
                        .map(|f| f.as_sql(true))
                        .collect::<Vec<_>>();
                    if list.len() > 0 {
                        param = format!("{} in ({})", key[0], list.join(","));
                    }
                }
                "like" => {
                    param = format!("{} LIKE :{}", key[0], lower_f);
                    set_params(lower_f, value)?;
                    index += 1;
                }
                _ => {
                    return Err(Error::CommonError { info: format!("Unsupported operator {}", f) });
                }
            };
            fmt_params.push(param);
        }

        Ok(fmt_params)
    }

    fn extra_op(map: &mut Map<String, JsValue>) -> Result<String, Error> {
        let mut opr = String::new();
        if let Some(val) = map.remove("operator") {
            let opr_s = val.as_str().ok_or(Error::CommonError { info: format!("invalid format at token operator:{}", val) })?;
            if opr_s == "AND" {
                opr.push_str(" AND ");
            } else if opr_s == "OR" {
                opr.push_str(" OR ");
            } else {
                return Err(Error::CommonError { info: format!("invalid operator:{}", opr_s) });
            }
        }
        Ok(opr)
    }

    fn get_js<T>(&self, token: T) -> Result<Map<String, JsValue>, Error>
        where
            T: ToString + Display {
        let values = self.body.get(token.to_string()).ok_or(Error::CommonError { info: "invalid json format".to_string() })?;
        let map = values.as_object().ok_or(Error::CommonError { info: format!("invalid json format at token '{}'", token) })?;
        Ok(map.clone())
    }

    fn parse_row(row: Row) -> HashMap<String, MyValue> {
        let mut map: HashMap<String, MyValue> = HashMap::with_capacity(row.len());
        for (i, c) in row.columns().iter().enumerate() {
            let v = row.as_ref(i);
            if let Some(v) = v {
                map.insert(c.name_str().as_ref().to_string(), v.clone());
            }
        }
        return map;
    }

    fn map2js(map: HashMap<String, MyValue>) -> JsValue {
        let mut m = Map::new();
        for (k,v) in map {
            m.insert(k, Self::my2js(v));
        }
        return JsValue::Object(m);
    }

    fn js2my(v: &JsValue) -> Option<MyValue> {
        match v {
            JsValue::String(s) => Some(MyValue::from(s)),
            JsValue::Number(n) => {
                if n.is_f64() {
                    Some(MyValue::from(n.as_f64().unwrap()))
                } else if n.is_i64() {
                    Some(MyValue::from(n.as_i64().unwrap()))
                } else if n.is_u64() {
                    Some(MyValue::from(n.as_u64().unwrap()))
                } else {
                    None
                }
            }
            _ => None
        }
    }

    fn my2js(v: MyValue) -> JsValue {
        match v {
            MyValue::Float(f) => json!(f),
            MyValue::Int(i) => json!(i),
            MyValue::UInt(u) => json!(u),
            MyValue::NULL => json!(null),
            _ => json!(v.as_sql(false).replace("'","")),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use dal::dao::Dao;
    use dal::table::Table;
    use dal::table::Field;
    use dal::dao::DML;
    use std::rc::Rc;
    use dal::db::DB;
    use dal::db::open_db;
    use std::sync::Arc;
    use std::sync::Mutex;
    use config::config::DBRoute;

    fn new(dml: DML, body: Value) -> Dao {
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

        Dao::new(Rc::new(table), dml, body)
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
    }

    #[test]
    fn test_access_delete() {
        let data = r##"{"RoleGuid__eq":"0000009b790008004b65fb","TwoKey__eq":2,"operator":"AND"}"##;
        let v: Value = serde_json::from_str(data).unwrap();
        let mut access = new(DML::Delete, v);

        let db = Arc::new(Mutex::new(get_db()));
        let exec_res = access.exec_sql(db).unwrap();

        println!("{}", exec_res);
    }

    #[test]
    fn test_access_update() {
        let data = r##"{"conditions":{"TwoKey__gte":1,"TwoKey__lte":9, "TwoKey__in":[21,31],"operator":"OR", "RoleGuid__like":"%9b%"},"values":{"CreateDate":"2017-02-23","CreateTimestamp":123}}"##;
        let v: Value = serde_json::from_str(data).unwrap();
        let mut access = new(DML::Update, v);

        let db = Arc::new(Mutex::new(get_db()));
        let exec_res = access.exec_sql(db).unwrap();

        println!("{}", exec_res);
    }

    #[test]
    fn test_access_select() {
        let data = r##"{"TwoKey__eq":3,"limit":3,"operator":"AND","order":"TwoKey__DESC,CreateTimestamp__ASC"}"##;
        let v: Value = serde_json::from_str(data).unwrap();
        let mut access = new(DML::Select, v);

        let db = Arc::new(Mutex::new(get_db()));
        let exec_res = access.exec_sql(db).unwrap();

        println!("{}", exec_res);
    }
}