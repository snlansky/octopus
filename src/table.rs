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
    pub fn new(db: String,
               model: String, ts: Vec<TableSchema>) -> Table {
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