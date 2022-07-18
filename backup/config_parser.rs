use tinyjson::JsonValue;
use std::fs;
use std::path::Path;
use colored::*;

pub fn parse() -> JsonValue{

    if !Path::new("./config.json").exists() {
        panic!("{}","Could not read config file './config.json'!".red());
    }

    let file = fs::read_to_string("./config.json").expect("Unable to read config file");
    //println!("{:?}", file);
    let config = file.parse().unwrap();
    return config
}