use std::collections::HashMap;

pub enum Value {
    PosInt(u64),
    NegInt(i64),
    Float(f64),
    String(String),
    Bool(bool),
}