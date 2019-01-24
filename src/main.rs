extern crate mysql;
extern crate serde_json;
use serde_json::{Result,Value};

mod db;
mod config;


fn main() {
    let data = r#"{"age":12,"name":"lucy"}"#;
    let v :Value= serde_json::from_str(data).unwrap();
    println!("Please call {} at the number {}", v["age"], v["name"]);
    match  &v {
        Value::Null => println!("null"),
        Value::Bool(b) => println!("bool {}", b),
        Value::Number(num) => println!("number {}", num),
        Value::String(s) => println!("string {}", s),
        Value::Array(v)=>println!("array {:?}", v),
        Value::Object(m)=> {
            for (k, v) in m {
                println!("map {} {:?}", k, v);
            }
        }
    }

    if let Value::Object(m) = v {
        for (k, v) in m {
            println!("map {} {:?}", k, v);
        }
    }

}

