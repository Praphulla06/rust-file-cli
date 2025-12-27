use std::env;

use file_manager::{
    append_to_file, create_file, delete_file, read_file, write_to_file,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Err(err) = run(&args) {
        eprintln!("{}", err);
    }
}

fn run(args: &[String]) -> Result<(), String> {
    let command = args.get(1).map(String::as_str)
        .ok_or("Please enter a command.")?;

    let filename = args.get(2)
        .ok_or("Please enter filename!")?;

    match command {
        "create" => create_file(filename),
        "read" => read_file(filename),
        "delete" => delete_file(filename),
        "write" => write_to_file(filename, &args[3..]),
        "append" => append_to_file(filename, &args[3..]),
        _ => return Err("Invalid command!".into()),
    }

    println!("Command: {}, File: {}", command, filename);
    Ok(())
}
