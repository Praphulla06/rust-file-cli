# Rust File CLI

A simple command-line tool written in Rust to perform basic file operations like **create**, **read**, **write**, **append**, and **delete**.

---

## Features

* **Create** a new file
* **Read** the contents of a file
* **Write** text to a file (overwrites existing content)
* **Append** text to an existing file
* **Delete** a file

---

## Requirements

* Rust (latest stable version recommended)
* Cargo package manager

---

## Installation

Clone this repository:

```bash
git clone https://github.com/<your-username>/rust-file-cli.git
cd rust-file-cli
```

Build the project:

```bash
cargo build --release
```

Run directly with Cargo:

```bash
cargo run -- <command> <filename> [content]
```

---

## Usage

```bash
cargo run -- <command> <filename> [content]
```

### Commands

| Command  | Description                         |
| -------- | ----------------------------------- |
| `create` | Create a new empty file             |
| `read`   | Display the contents of a file      |
| `write`  | Overwrite a file with new content   |
| `append` | Append content to the end of a file |
| `delete` | Delete a file                       |

### Examples

Create a new file:

```bash
cargo run -- create test.txt
```

Write content to a file:

```bash
cargo run -- write test.txt "Hello, Rust!"
```

Append content to a file:

```bash
cargo run -- append test.txt "This will be added."
```

Read file contents:

```bash
cargo run -- read test.txt
```

Delete a file:

```bash
cargo run -- delete test.txt
```
