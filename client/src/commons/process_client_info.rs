use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;

pub fn process_client_info() -> Result<HashMap<String, String>, ()> {

    let client = process_config("datos.txt");

    Ok(client.unwrap())
}

fn process_config(filename: &str) -> Result<HashMap<String, String>, ()> {
    let contents = fs::read_to_string(filename).expect("error");
    let mut client_info = HashMap::new();

    let split = contents.split("\n");
    let vector1: Vec<&str> = split.collect();

    for parameter in vector1 {
        let split2 = parameter.split(": ");
        let parsed_info: Vec<&str> = split2.collect();
        client_info.insert(parsed_info[0].to_string(), parsed_info[1].to_string());
    }

    return Ok(client_info);
}
