use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = match args.get(1) {
        Some(cmd) => cmd.as_str(),
        None => {
            println!("Please enter a command.");
            return;
        }
    };

    match command {
        "create" | "read" | "delete" | "write" | "append" => {
            let filename = match args.get(2) {
                Some(file) => file,
                None => {
                    println!("Please enter filename!");
                    return;
                }
            };

            match command {
                "create" => {
                    match File::create(filename) {
                        Ok(_) => println!("File '{}' created successfully.", filename),
                        Err(e) => println!("Error creating file: {}", e),
                    }
                }

                "read" => {
                    let mut file = match File::open(filename) {
                        Ok(file) => file,
                        Err(e) => {
                            println!("Error opening file: {}", e);
                            return;
                        }
                    };

                    let mut contents = String::new();
                    if let Err(e) = file.read_to_string(&mut contents) {
                        println!("Error reading file: {}", e);
                        return;
                    }

                    println!("File contents:\n{}", contents);
                }

                "delete" => {
                    match fs::remove_file(filename) {
                        Ok(_) => println!("File '{}' deleted successfully.", filename),
                        Err(e) => println!("Error deleting file: {}", e),
                    }
                }

                "write" => {
                    let content = match args.get(3..) {
                        Some(content) if !content.is_empty() => content.join(" "),
                        _ => {
                            println!("Enter content you want to write...");
                            return;
                        }
                    };

                    match fs::write(filename, &content) {
                        Ok(_) => println!("Wrote '{}' to file '{}'.", content, filename),
                        Err(e) => println!("Error writing to file: {}", e),
                    }
                }

                "append" => {
                    let content = match args.get(3..) {
                        Some(content) if !content.is_empty() => content.join(" "),
                        _ => {
                            println!("Enter content you want to append...");
                            return;
                        }
                    };

                    let mut file = match OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(filename)
                    {
                        Ok(file) => file,
                        Err(e) => {
                            println!("Error opening file: {}", e);
                            return;
                        }
                    };

                    if let Err(e) = writeln!(file, "{}", content) {
                        println!("Error appending to file: {}", e);
                        return;
                    }

                    println!("Appended '{}' to file '{}'.", content, filename);
                }

                _ => println!("Invalid command"),
            }

            println!("Command: {}, File: {}", command, filename);
        }

        _ => {
            println!("Invalid Command!");
        }
    }
}
