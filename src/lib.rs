use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};

pub fn create_file(filename: &str) {
    if let Err(e) = File::create(filename) {
        eprintln!("Error creating file: {}", e);
    } else {
        println!("File '{}' created successfully.", filename);
    }
}

pub fn read_file(filename: &str) {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return;
        }
    };

    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        eprintln!("Error reading file: {}", e);
        return;
    }

    println!("File contents:\n{}", contents);
}

pub fn delete_file(filename: &str) {
    if let Err(e) = fs::remove_file(filename) {
        eprintln!("Error deleting file: {}", e);
    } else {
        println!("File '{}' deleted successfully.", filename);
    }
}

pub fn write_to_file(filename: &str, content: &[String]) {
    let content = match extract_content(content) {
        Some(c) => c,
        None => return,
    };

    if let Err(e) = fs::write(filename, &content) {
        eprintln!("Error writing to file: {}", e);
    } else {
        println!("Wrote '{}' to file '{}'.", content, filename);
    }
}

pub fn append_to_file(filename: &str, content: &[String]) {
    let content = match extract_content(content) {
        Some(c) => c,
        None => return,
    };

    let mut file = match OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)
    {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return;
        }
    };

    if let Err(e) = writeln!(file, "{}", content) {
        eprintln!("Error appending to file: {}", e);
    } else {
        println!("Appended '{}' to file '{}'.", content, filename);
    }
}

fn extract_content(args: &[String]) -> Option<String> {
    if args.is_empty() {
        eprintln!("Enter content!");
        None
    } else {
        Some(args.join(" "))
    }
}
