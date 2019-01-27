use std::collections::HashMap;
use std::any::Any;
use std::fmt::Display;

pub enum Value {
    PosInt(u64),
    NegInt(i64),
    Float(f64),
    String(String),
    Bool(bool),
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::PosInt(i) => i.to_string(),
            Value::NegInt(i) => i.to_string(),
            Value::Float(i) => i.to_string(),
            Value::String(s) => s.clone(),
            Value::Bool(b) => {
                if *b {
                    "TRUE".to_string()
                } else {
                    "FALSE".to_string()
                }
            }
        }
    }
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
}