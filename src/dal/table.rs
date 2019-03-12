use dal::lua::LuaScript;
use dal::value::ConvertTo;
use error::Error;
use redis::Connection;
use serde_json::Value as JsValue;
use std::collections::HashMap;

#[allow(dead_code)]
const METADATA: &str = "metadata";
const SCHEMA: &str = "scheme";
const PRIMARY_KEY: &str = "pk";
const SET: &str = "set";
const TIMER: &str = "timer";
const COUNTER: &str = "counter";
const FIELD_SEP: &str = ",";

#[derive(Debug)]
pub struct TableSchema {
    pub table_name: String,
    pub column_name: String,
    pub data_type: String,
    pub column_key: String,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub tpe: String,
}

impl Field {
    pub fn get_value(&self, s: &str) -> Result<JsValue, Error> {
        match self.tpe.as_str() {
            "char" => Ok(json!(s)),
            "varchar" => Ok(json!(s)),
            "text" => Ok(json!(s)),
            "tinytext" => Ok(json!(s)),
            "mediumtext" => Ok(json!(s)),
            "longtext" => Ok(json!(s)),
            "date" => Ok(json!(s)),
            "time" => Ok(json!(s)),
            "datetime" => Ok(json!(s)),
            "timestamp" => Ok(json!(s)),
            "int" => {
                let i = s.parse::<i64>().map_err(|_| Error::CommonError {
                    info: format!(
                        "field: {} [{}] convert to {} failed",
                        self.name, s, self.tpe
                    ),
                })?;
                Ok(json!(i))
            }
            "tinyint" => {
                let i = s.parse::<i8>().map_err(|_| Error::CommonError {
                    info: format!(
                        "field: {} [{}] convert to {} failed",
                        self.name, s, self.tpe
                    ),
                })?;
                Ok(json!(i))
            }
            "smallint" => {
                let i = s.parse::<i16>().map_err(|_| Error::CommonError {
                    info: format!(
                        "field: {} [{}] convert to {} failed",
                        self.name, s, self.tpe
                    ),
                })?;
                Ok(json!(i))
            }
            "mediumint" => {
                let i = s.parse::<i32>().map_err(|_| Error::CommonError {
                    info: format!(
                        "field: {} [{}] convert to {} failed",
                        self.name, s, self.tpe
                    ),
                })?;
                Ok(json!(i))
            }
            "bigint" => {
                let i = s.parse::<i64>().map_err(|_| Error::CommonError {
                    info: format!(
                        "field: {} [{}] convert to {} failed",
                        self.name, s, self.tpe
                    ),
                })?;
                Ok(json!(i))
            }
            "float" => {
                let i = s.parse::<f32>().map_err(|_| Error::CommonError {
                    info: format!(
                        "field: {} [{}] convert to {} failed",
                        self.name, s, self.tpe
                    ),
                })?;
                Ok(json!(i))
            }
            "double" => {
                let i = s.parse::<f64>().map_err(|_| Error::CommonError {
                    info: format!(
                        "field: {} [{}] convert to {} failed",
                        self.name, s, self.tpe
                    ),
                })?;
                Ok(json!(i))
            }
            "decimal" => {
                let i = s.parse::<f64>().map_err(|_| Error::CommonError {
                    info: format!(
                        "field: {} [{}] convert to {} failed",
                        self.name, s, self.tpe
                    ),
                })?;
                Ok(json!(i))
            }
            _ => Err(Error::CommonError {
                info: format!(
                    "field: {} [{}] convert to undefined {} error",
                    self.name, s, self.tpe
                ),
            }),
        }
    }
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
            fields.push(Field {
                name: f.column_name.clone(),
                tpe: f.data_type.clone(),
            })
        }
        Table {
            db,
            model,
            pks,
            fields,
        }
    }

    pub fn default(db: String, model: String, pks: Vec<String>, fields: Vec<Field>) -> Table {
        Table {
            db,
            model,
            pks,
            fields,
        }
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

    pub fn get_db(&self) -> String {
        self.db.clone()
    }

    pub fn get_model(&self) -> String {
        self.model.clone()
    }

    pub fn get_pks(&self) -> &Vec<String> {
        &self.pks
    }

    pub fn get_fields(&self) -> &Vec<Field> {
        &self.fields
    }

    pub fn get_model_key(&self, row: &HashMap<String, JsValue>) -> String {
        let pv_list = self
            .pks
            .iter()
            .map(|p| {
                let value = row.get(p);
                if let Some(v) = value {
                    v.convert()
                } else {
                    "".to_string()
                }
            })
            .collect::<Vec<_>>();

        format!(
            "{}:{}:{}:{}",
            self.db,
            self.model,
            self.pks.join(FIELD_SEP),
            pv_list.join(FIELD_SEP)
        )
    }

    pub fn register_schema(&self, con: &Connection) -> Result<(), redis::RedisError> {
        let mut script = LuaScript::new();
        script.sadd(self.get_db_set_key(), vec![self.model.clone()]);
        let fv = self
            .fields
            .iter()
            .map(|Field { name: n, tpe: t }| (n.clone(), t.clone()))
            .collect::<HashMap<_, _>>();
        script.hmset(self.get_table_schema_key(), fv);
        script.invoke(con)?;
        Ok(())
    }
}

pub fn set_hash_values(fv: &HashMap<String, JsValue>) -> HashMap<String, String> {
    fv.iter()
        .map(|(k, v)| (k.clone(), v.to_string()))
        .collect::<HashMap<String, String>>()
}

#[cfg(test)]
mod tests {
    use dal::table::Field;
    use dal::table::Table;
    use serde_json::Value as JsValue;
    use std::collections::HashMap;

    #[test]
    fn test_set_hash_values() {
        let mut hash: HashMap<String, JsValue> = HashMap::new();
        hash.insert(String::from("k1"), json!(12));
        hash.insert(String::from("k2"), json!(23.5));
        hash.insert(String::from("k3"), json!("hello"));
        hash.insert(String::from("k4"), json!(true));

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
        row.insert(String::from("name"), json!("lucy"));
        row.insert(String::from("age"), json!(12));
        row.insert(String::from("tel"), json!("129099101"));
        let tbl = Table::default(db_name, model_name, pl, vec![]);
        assert_eq!(tbl.get_model_key(&row), res);
    }

    #[test]
    fn test_register_schema() {
        let client = redis::Client::open("redis://:snlan@www.snlan.top:6379/").unwrap();
        let con = client.get_connection().unwrap();
        let table = Table {
            db: "block".to_string(),
            model: "user".to_string(),
            pks: vec!["name".to_string(), "age".to_string()],
            fields: vec![
                Field {
                    name: "name".to_string(),
                    tpe: "vchar".to_string(),
                },
                Field {
                    name: "age".to_string(),
                    tpe: "int".to_string(),
                },
                Field {
                    name: "addr".to_string(),
                    tpe: "text".to_string(),
                },
            ],
        };

        let ret = table.register_schema(&con);
        match ret {
            Err(e) => panic!(e),
            _ => {}
        }
    }
}
