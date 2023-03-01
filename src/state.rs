use std::fs::{self, File};
use std::io::Read;

use serde_json::json;
use serde_json::value::Value;
use serde_json::Map;

// 读取文件
pub fn read_file(file_name: &str) -> Map<String, Value> {
    let mut file = File::open(file_name).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    if buffer.len() == 0 {
        buffer.push_str("{}")
    }
    let json: Value = serde_json::from_str(&buffer).unwrap();
    let state = json.as_object().unwrap().clone();
    return state;
}

// 写入文件
pub fn write_file(file_name: &str, state: &mut Map<String, Value>) {
    let new_data = json!(state);
    fs::write(file_name, new_data.to_string()).expect("unable to write to file");
}

// 清空文件
pub fn clean_file(file_name: &str) {
    fs::write(file_name, "").expect("unable");
}
