# Terminal-based authentication system

A simple command-line authentication system written in Rust.  
This program allows users to sign up, sign in, and store credentials in a local file using a terminal-based interface.

---

## Features

- Interactive command-line menu
- Sign-up and sign-in functionality
- Credentials stored in `password.txt`
- hashing system using sha256 (fast algorithm) with a short salt to show how this impact the security 
- Basic validation and failed-attempt tracking
---

## Requirements

- Rust (stable)
- Cargo

If Rust is not installed, download it here:  
https://www.rust-lang.org/tools/install

## Cloning the Project

Use Git to clone the repository: `https://github.com/wanesaf/infosec.git`

## Build the project 
To compile the project:
 - `cargo build`
you can then run the program from `/target/debug/`

## Running the Application

Run directly with Cargo: `cargo run`
