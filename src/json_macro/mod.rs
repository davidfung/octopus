use std::collections::HashMap;

pub const DESC: &str = "Json Macro";

#[derive(Clone, PartialEq, Debug)]
enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>)
}

macro_rules! json {
    (Null) => { Json::Null }
}

#[test]
fn json_null() {
    assert_eq!(json!(Null), Json::Null);
}

pub fn entry() {
    println!("json macro");
}

