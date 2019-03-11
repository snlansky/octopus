use mysql::Value as MyValue;
use serde_json::Map;
use serde_json::Value as JsValue;
use std::collections::HashMap;

pub trait ConvertTo<T> {
    fn convert(&self) -> T;
}

impl ConvertTo<JsValue> for JsValue {
    fn convert(&self) -> JsValue {
        self.clone()
    }
}

impl ConvertTo<JsValue> for i32 {
    fn convert(&self) -> JsValue {
        json!(self)
    }
}

impl ConvertTo<String> for JsValue {
    fn convert(&self) -> String {
        match self {
            JsValue::String(s) => s.clone(),
            JsValue::Number(n) => {
                if n.is_f64() {
                    format!("{}", n.as_f64().unwrap())
                } else if n.is_i64() {
                    format!("{}", n.as_i64().unwrap())
                } else if n.is_u64() {
                    format!("{}", n.as_u64().unwrap())
                } else {
                    "0".to_string()
                }
            }
            _ => self.to_string(),
        }
    }
}

// Map<String, JsValue>-> HashMap<String,String>
impl ConvertTo<HashMap<String, String>> for Map<String, JsValue> {
    fn convert(&self) -> HashMap<String, String> {
        let mut hm = HashMap::new();
        for (k, v) in self {
            let s: String = v.convert();
            hm.insert(k.clone(), s);
        }
        hm
    }
}

impl ConvertTo<Map<String, JsValue>> for HashMap<String, JsValue> {
    fn convert(&self) -> Map<String, JsValue> {
        let mut m = Map::with_capacity(self.len());

        for (k, v) in self {
            m.insert(k.clone(), v.clone());
        }
        m
    }
}

// Map<String, JsValue>-> HashMap<String,String>
impl ConvertTo<HashMap<String, JsValue>> for Map<String, JsValue> {
    fn convert(&self) -> HashMap<String, JsValue> {
        let mut hm = HashMap::new();
        for (k, v) in self {
            hm.insert(k.clone(), v.clone());
        }
        hm
    }
}

// MyValue -> JsValue
impl ConvertTo<JsValue> for MyValue {
    fn convert(&self) -> JsValue {
        match self {
            MyValue::Float(f) => json!(f),
            MyValue::Int(i) => json!(i),
            MyValue::UInt(u) => json!(u),
            MyValue::NULL => json!(null),
            _ => json!(self.as_sql(false).replace("'", "")),
        }
    }
}

// HashMap<String, MyValue> -> JsValue
impl ConvertTo<JsValue> for HashMap<String, MyValue> {
    fn convert(&self) -> JsValue {
        let mut m = Map::new();
        for (k, v) in self {
            m.insert(k.clone(), v.convert());
        }
        return JsValue::Object(m);
    }
}

// JsValue -> Option<MyValue>
impl ConvertTo<Option<MyValue>> for JsValue {
    fn convert(&self) -> Option<MyValue> {
        match self {
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
            _ => None,
        }
    }
}
