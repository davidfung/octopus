use std::collections::HashMap;

use crate::menu::MenuItem;

pub fn menu() -> MenuItem {
    MenuItem{task: entry, desc: "Json Macro"}
}

#[allow(dead_code)]
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

fn json_null() {
    assert_eq!(json!(Null), Json::Null);
}

pub fn entry() {
    println!("json macro");
    json_null();
}

