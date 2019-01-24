use std::collections::HashMap;
use std::any::Any;
use std::fmt::Display;

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

pub fn get_model_key<T: Any + Display>(db_name: &String,
                                       model_name: &String,
                                       pk_list: &Vec<String>,
                                       row: &HashMap<String, T>) -> String {
    "Fs".to_string()
}

pub fn set_hash_values<T: Any + Display>(fv: &HashMap<String, T>) -> HashMap<String, String> {
//    let hash: HashMap<String, String> = HashMap::new();
    fv.iter().map(|(k, v)| {
        (k.clone(), conv_string(v))
    }).collect::<HashMap<String, String>>()
}


pub fn conv_string<T: Any + Display>(v: &T) -> String {
    let t = v as &dyn Any;
    if let Some(s) = t.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = t.downcast_ref::<&str>() {
        s.to_string()
    } else if let Some(i) = t.downcast_ref::<i32>() {
        i.to_string()
    } else if let Some(i) = t.downcast_ref::<i64>() {
        i.to_string()
    } else if let Some(i) = t.downcast_ref::<u32>() {
        i.to_string()
    } else if let Some(i) = t.downcast_ref::<u64>() {
        i.to_string()
    } else if let Some(i) = t.downcast_ref::<f32>() {
        i.to_string()
    } else if let Some(i) = t.downcast_ref::<f64>() {
        i.to_string()
    } else if let Some(i) = t.downcast_ref::<bool>() {
        if *i {
            "TRUE".to_string()
        } else {
            "FALSE".to_string()
        }
    } else {
        format!("{}", v)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::any::Any;

    #[test]
    fn test_conv_string() {
        use super::conv_string;
        assert_eq!(conv_string(&12), "12");
        assert_eq!(conv_string(&12.2), "12.2");
        assert_eq!(conv_string(&false), "FALSE");
        assert_eq!(conv_string(&"string"), "string");
        let s = String::from("hello");
        assert_eq!(conv_string(&s), s);
    }
//
//    #[test]
//    fn test_set_hash_values() {
//        let mut hash :HashMap<String, Any> = HashMap::new();
//        hash.insert(String::from("k1"), Box::new(12));
////        hash.insert(String::from("k2"), 12.5);
////        hash.insert(String::from("k3"), "abc");
////        hash.insert(String::from("k4"), true);
//
//        let new = super::set_hash_values(&hash);
//        for (k, v) in &new {
//            println!("{} {}", k, v);
//        }
//    }
}