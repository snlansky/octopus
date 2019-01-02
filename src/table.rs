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
pub struct Table {
    db: String,
    model: String,
    pks: Vec<String>,
    field_names: Vec<String>,
    field_types: Vec<String>,
}

impl Table {
    pub fn new(db: &str,
               model: &str,
               pks: Vec<&str>,
               field_names: Vec<&str>,
               field_types: Vec<&str>) -> Table {
        Table {
            db: String::from(db),
            model: String::from(model),
            pks: pks.iter().map(|&elem| String::from(elem)).collect::<Vec<_>>(),
            field_names: field_names.iter().map(|&elem| String::from(elem)).collect::<Vec<_>>(),
            field_types: field_types.iter().map(|&elem| String::from(elem)).collect::<Vec<_>>(),
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
}