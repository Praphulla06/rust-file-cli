use std::env;

use file_manager::{append_to_file, create_file, delete_file, read_file, write_to_file};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(String::as_str) {
        Some(command) => {
            let filename = match args.get(2) {
                Some(file) => file,
                None => {
                    println!("Please enter a filename");
                    return;
                }
            };
            match command {
                "create" => create_file(filename),
                "read" => read_file(filename),
                "delete" => delete_file(filename),
                "write" => write_to_file(filename, &args[3..]),
                "append" => append_to_file(filename, &args[3..]),
                _ => {
                    println!("Invalid command");
                    return;
                }
            }
        }
        None => {
            println!("No command found!");
            return;
        }
    };
}
