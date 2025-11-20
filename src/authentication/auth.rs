use core::{panic, time};
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
    process::exit,
    thread,
};

pub struct Credential {
    pub username: String,
    pub salt: String,
    pub hash: String,
}

use crate::authentication::utils;

pub fn sign_up() -> Credential {
    let username = loop {
        print!("username:");
        io::stdout().flush().unwrap();
        let username: String = utils::read_input();
        let result = username
            .chars()
            .filter(|x| x.is_alphabetic())
            .all(|x| x.is_lowercase());
        if result {
            if username.len() != 5 {
                println!("Must be 5 characters");
            } else {
                break username;
            }
        } else {
            println!("Not all characters are lowercase");
        }
    };
    let password = loop {
        print!("password:");
        io::stdout().flush().unwrap();
        let password: String = utils::read_input();
        let result = password.chars().all(|x| x.is_ascii_alphanumeric());
        if result {
            if password.len() != 8 {
                println!("Must be 8 characters");
            } else {
                break password;
            }
        } else {
            println!("chars must be letters or digits");
        }
    };
    let salt = utils::gen_salt(5);
    let hashed = utils::hash(&password, &salt);
    // println!("salt is {}", salt);
    //println!("hash is {}", hashed);
    let c = Credential {
        username: username,
        salt: salt,
        hash: hashed,
    };
    c
}

pub fn sign_in(fpath: &str, fail: &mut u32) {
    print!("username:");
    io::stdout().flush().unwrap();
    let username: String = utils::read_input();
    print!("password:");
    io::stdout().flush().unwrap();
    let password: String = utils::read_input();
    match File::open(fpath) {
        Ok(f) => {
            let reader = BufReader::new(&f);
            for line in reader.lines() {
                let line = match line {
                    Ok(line) => line,
                    Err(_) => panic!("Error when trying to read content"),
                };
                let result = utils::verify_pass(&username, &password, &line);
                if result {
                    println!("Welcome home ! ");
                    return;
                }
            }
        }
        Err(_) => {}
    };
    println!("Invalid username or password");
    *fail += 1;
    match fail {
        3 => {
            println!("You are blocked for 5 seconds");
            thread::sleep(time::Duration::from_secs(5));
        }
        6 => {
            println!("You are blocked for 10 seconds");
            thread::sleep(time::Duration::from_secs(10));
        }
        9 => {
            println!("You are blocked for 15 seconds");
            thread::sleep(time::Duration::from_secs(10));
        }
        12 => {
            println!("You are blocked for ever :(");
            exit(0);
        }
        _ => {} //unreachable!
    }
}
