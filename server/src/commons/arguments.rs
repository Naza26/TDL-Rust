use std::fs;

static SERVER_ARGS: usize = 2;

pub fn process_arguments(argv: Vec<String>, config: &mut Vec<String>) -> Result<(), ()> {
    if argv.len() != SERVER_ARGS {
        println!(
            "{:?}{}{:?}",
            SERVER_ARGS,
            "arguments were expected but received",
            argv.len()
        );
        return Err(());
    }

    process_config(&argv[1], config);
    if config.len() != 1 {
        println!("{}{:?}", "1 lines were expected in config.txt but received", config.len());
        return Err(());
    }

    Ok(())
}

fn process_config(filename: &str, config: &mut Vec<String>) {
    let contents = fs::read_to_string(filename).expect("error");

    let split = contents.split("\n");
    let vector1: Vec<&str> = split.collect();

    for parameter in vector1 {
        let split2 = parameter.split(": ");
        let vector2: Vec<&str> = split2.collect();
        config.push(vector2[1].to_string());
    }
}
