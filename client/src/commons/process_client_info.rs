use std::collections::HashMap;
use std::fs;

pub fn process_client_info() -> Result<HashMap<String, String>, ()> {

    let client = process_client_configuration_from("datos.txt");

    Ok(client.unwrap())
}

fn process_client_configuration_from(filename: &str) -> Result<HashMap<String, String>, ()> {
    let file_content = fs::read_to_string(filename).expect("error");
    let mut client_info = HashMap::new();

    let lines = file_content.split('\n');
    let lines_as_vector: Vec<&str> = lines.collect();

    for line in lines_as_vector {
        let line_parts = line.split(": ");
        let parsed_info: Vec<&str> = line_parts.collect();
        client_info.insert(parsed_info[0].to_string(), parsed_info[1].to_string());
    }

    return Ok(client_info);
}

