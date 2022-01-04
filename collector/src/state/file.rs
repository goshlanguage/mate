use serde_json::{json, value::Value, Map};
use std::{fs, fs::File, io::Read};

pub fn read_file(file_name: &str) -> Value {
    let mut file = File::open(file_name).unwrap();
    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();

    let json: Value = serde_json::from_str(&data).unwrap();
    json
}

pub fn read_map_from_file(file_name: &str) -> Map<String, Value> {
    let mut file = File::open(file_name).unwrap();
    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();

    let json: Value = serde_json::from_str(&data).unwrap();
    json.as_object().unwrap().clone()
}

pub fn write_file(file_name: &str, state: Value) {
    let new_data = json!(state);
    fs::write(file_name, new_data.to_string()).expect("Failed to write file");
}

pub fn write_map_to_file(file_name: &str, state: &Map<String, Value>) {
    let new_data = json!(state);
    fs::write(file_name, new_data.to_string()).expect("Failed to write file");
}
