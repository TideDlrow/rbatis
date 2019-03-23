extern crate serde_json;

use serde_json::json;
use serde_json::Value;
use serde::{Serialize, Deserialize};
use chrono::Local;
use crate::utils::time_util;
use std::iter::Map;
use std::any::Any;

pub fn Eval(left: &Value,
            right: &Value,
            op: &str) -> (Value, String) {


//    let is_left_number = left.is_f64();
//    let is_right_number = right.is_f64();
//    let is_left_string = left.is_string();
//    let is_right_string = right.is_string();
//    let is_left_null = left.is_null();
//    let is_right_null = right.is_null();

    if op == "&&"{
        return (Value::Bool(left.as_bool().unwrap() && right.as_bool().unwrap()),String::new());
    }
    if op == "||"{
        return (Value::Bool(left.as_bool().unwrap() || right.as_bool().unwrap()),String::new());
    }

    if op == "==" {
        return (Value::Bool(left.eq(right)), String::new());
    }
    if op == "!=" {
        return (Value::Bool(!left.eq(right)), String::new());
    }
    if op == ">=" {
        let booll = left.is_f64();
        let boolr = right.is_f64();
        if booll && boolr {
            return (Value::Bool(left.as_f64() >= right.as_f64()), String::new());
        }
    }
    if op == "<=" {
        let booll = left.is_f64();
        let boolr = right.is_f64();
        if booll && boolr {
            return (Value::Bool(left.as_f64() <= right.as_f64()), String::new());
        }
    }
    if op == ">" {
        let booll = left.is_f64();
        let boolr = right.is_f64();
        if booll && boolr {
            return (Value::Bool(left.as_f64() > right.as_f64()), String::new());
        }
    }
    if op == "<" {
        let booll = left.is_f64();
        let boolr = right.is_f64();
        if booll && boolr {
            return (Value::Bool(left.as_f64() < right.as_f64()), String::new());
        }
    }
    if op == "+" {
        let booll = left.is_f64();
        let boolr = right.is_f64();
        if booll && boolr {
            return (Value::Number(serde_json::Number::from_f64(left.as_f64().unwrap() + right.as_f64().unwrap()).unwrap()), String::new());
        }else{
            let mut s = String::new();
            s.push_str(left.as_str().unwrap());
            s.push_str(right.as_str().unwrap());
            return (Value::from(s), String::new());
        }
    }
    if op == "-" {
        let booll = left.is_f64();
        let boolr = right.is_f64();
        if booll && boolr {
            return (Value::Number(serde_json::Number::from_f64(left.as_f64().unwrap() - right.as_f64().unwrap()).unwrap()), String::new());
        }
    }
    if op == "*" {
        let booll = left.is_f64();
        let boolr = right.is_f64();
        if booll && boolr {
            return (Value::Number(serde_json::Number::from_f64(left.as_f64().unwrap() * right.as_f64().unwrap()).unwrap()), String::new());
        }
    }
    if op == "/" {
        let booll = left.is_f64();
        let boolr = right.is_f64();
        if booll && boolr {
            return (Value::Number(serde_json::Number::from_f64(left.as_f64().unwrap() / right.as_f64().unwrap()).unwrap()), String::new());
        }
    }
    return (Value::Null, String::new());
}


#[test]
fn TestParser() {
    let john = json!({
        "name": "John Doe",
        "age": Value::Null,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });


    let age = &john["age"];
    println!("{}", *age);
}

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[test]
fn TestTakeValue() {
    let point = Point { x: 1, y: 2 };

    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {}", serialized);

    //create serde_json::Value
    let john = json!(point);
    println!("{}", john["x"]);

    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}

#[test]
fn BenchmarkFromStr() {
    let point = Point { x: 1, y: 2 };

    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {}", serialized);

    let total = 100000;
    let now = Local::now();
    for i in 0..total {
        let deserialized: Point = serde_json::from_str(&serialized).unwrap();
        // println!("deserialized = {:?}", deserialized);
    }
    time_util::count_time(total, now);
    time_util::count_tps(total, now);
}

#[test]
fn BenchmarkToString() {
    let point = Point { x: 1, y: 2 };


    let total = 100000;
    let now = Local::now();
    for i in 0..total {
        let serialized = serde_json::to_string(&point).unwrap();
        let deserialized: Value = serde_json::from_str(&serialized).unwrap();
    }
    time_util::count_time(total, now);
    time_util::count_tps(total, now);
}