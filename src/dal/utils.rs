use serde_json::Value as JsValue;
use std::collections::HashMap;
use serde_json::Map;

pub fn js2string(js: &JsValue) -> String {
    match js {
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
        _ => js.to_string()
    }
}

pub fn map2hashmap(map :&Map<String, JsValue>) -> HashMap<String,String> {
    let mut hm = HashMap::new();
    for (k, v) in map {
        hm.insert(k.clone(), js2string(v));
    }
    hm
}