use std::fs;

static SERVER_ARGS: usize = 2;

pub fn arguments_cant_be_processed(argv: Vec<String>, config: &mut Vec<String>) -> bool {
    if argv.len() != SERVER_ARGS {
        println!(
            "{:?}arguments were expected but received{:?}",
            SERVER_ARGS,
            argv.len()
        );
        return true;
    }

    process_config(&argv[1], config);
    if config.len() != 1 {
         println!("1 lines were expected in config.txt but received{:?}", config.len());
        return true;
    }

    false
}

fn process_config(filename: &str, config: &mut Vec<String>) {
    let file_content = fs::read_to_string(filename).expect("error");

    let lines = file_content.split('\n');
    let lines_as_vector: Vec<&str> = lines.collect();

    for line in lines_as_vector {
        let line_parts = line.split(": ");
        let parsed_info: Vec<&str> = line_parts.collect();
        config.push(parsed_info[1].to_string());
    }
}
