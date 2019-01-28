use std::collections::HashMap;
use super::value::Value;

#[allow(dead_code)]
const METADATA: &str = "metadata";
const SCHEMA: &str = "scheme";
const PRIMARY_KEY: &str = "pk";
const SET: &str = "set";
const TIMER: &str = "timer";
const COUNTER: &str = "counter";
const LUA_RET: &str = "total";
const FIELD_SEP: &str = ",";

#[derive(Debug)]
pub struct TableSchema {
    pub table_name: String,
    pub column_name: String,
    pub data_type: String,
    pub column_key: String,
}

#[derive(Debug)]
pub struct Field {
    name: String,
    tpe: String,
}


#[derive(Debug)]
pub struct Table {
    db: String,
    model: String,
    pks: Vec<String>,
    fields: Vec<Field>,
}

impl Table {
    pub fn new(db: String, model: String, ts: Vec<TableSchema>) -> Table {
        let mut pks = Vec::new();
        let mut fields = Vec::new();

        for f in ts {
            if f.column_key == "PRI" {
                pks.push(f.column_name.clone())
            }
            fields.push(Field { name: f.column_name.clone(), tpe: f.data_type.clone() })
        }

        Table { db, model, pks, fields }
    }

    // 生成缓存对象集合
    pub fn get_db_set_key(&self) -> String {
        format!("{}:{}:{}", self.db, METADATA, SET)
    }

    // 生成缓存对象模式键名
    pub fn get_table_schema_key(&self) -> String {
        format!("{}:{}:{}:{}", self.db, self.model, METADATA, SCHEMA)
    }

    // 生成缓存对象主键名
    pub fn get_table_pk_key(&self) -> String {
        format!("{}:{}:{}:{}", self.db, self.model, METADATA, PRIMARY_KEY)
    }

    // 生成缓存对象标识集合
    pub fn get_table_set_key(&self) -> String {
        format!("{}:{}:{}:{}", self.db, self.model, METADATA, SET)
    }

    // 生成缓存对象访问计时键名
    pub fn get_table_timer_key(&self) -> String {
        format!("{}:{}:{}:{}", self.db, self.model, METADATA, TIMER)
    }

    // 生成缓存对象访问计数键名
    pub fn get_table_counter_key(&self) -> String {
        format!("{}:{}:{}:{}", self.db, self.model, METADATA, COUNTER)
    }
}

pub fn get_model_key(db_name: &String,
                     model_name: &String,
                     pk_list: &Vec<String>,
                     row: &HashMap<String, Value>) -> String {
    let pv_list = pk_list.iter().map(|p|{
        let value = row.get(p);
        if let Some(v) = value {
            v.to_string()
        } else {
            "".to_string()
        }
    }).collect::<Vec<_>>();

    format!("{}:{}:{}:{}", db_name, model_name, pk_list.join(FIELD_SEP), pv_list.join(FIELD_SEP))
}

pub fn set_hash_values(fv: &HashMap<String, Value>) -> HashMap<String, String> {
    fv.iter().map(|(k, v)| {
        (k.clone(), v.to_string())
    }).collect::<HashMap<String, String>>()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use db::value::Value;

    #[test]
    fn test_set_hash_values() {
        let mut hash: HashMap<String, Value> = HashMap::new();
        hash.insert(String::from("k1"), Value::NegInt(12));
        hash.insert(String::from("k2"), Value::Float(23.5));
        hash.insert(String::from("k3"), Value::String(String::from("hello")));
        hash.insert(String::from("k4"), Value::Bool(true));

        let new = super::set_hash_values(&hash);
        for (k, v) in &new {
            println!("---{} {:?}", k, v);
        }
    }

    #[test]
    fn test_get_model_key() {
        let res = String::from("daqing:user_table:name,age:lucy,12");
        let db_name = String::from("daqing");
        let model_name = String::from("user_table");
        let pl = vec![String::from("name"), String::from("age")];
        let mut row = HashMap::new();
        row.insert(String::from("name"), Value::String("lucy".to_string()));
        row.insert(String::from("age"), Value::NegInt(12));
        row.insert(String::from("tel"), Value::String("129099101".to_string()));
        assert_eq!(super::get_model_key(&db_name, &model_name, &pl, &row), res);
    }
}
