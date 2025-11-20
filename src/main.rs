use core::{panic, time};
use rand::{rng, Rng};
use sha2::{Digest, Sha256};
use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    process::exit,
    str::FromStr,
    thread,
};
struct Credential {
    username: String,
    salt: String,
    hash: String,
}

fn read_input<T: FromStr>() -> T {
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Try again!");
            continue;
        }

        match input.trim().parse::<T>() {
            Ok(value) => return value,
            Err(_) => {}
        }
    }
}

fn gen_salt(len: usize) -> String {
    let mut r = rng();
    (0..len)
        .map(|_| r.random_range(0..10).to_string())
        .collect()
}

fn hash(password: &str, salt: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(format!("{}{}", password, salt));
    let r = hasher.finalize();
    format!("{:x}", r)
} // argon2 require 8 bytes , here 5 must choose another algorithm , sha256 is not slow, maybe
  // iterations will be a good idea
  //
  //
  //
  //
fn verify_pass(usr: &str, pass: &str, cred: &str) -> bool {
    let split = cred.split('#');
    let collection = split.collect::<Vec<&str>>();
    let username = collection[0];
    let salt = collection[1];
    let h = collection[2];
    if username != usr {
        return false;
    } else {
        let hashed = hash(&pass, &salt);
        if hashed == h {
            return true;
        }
        false
    }
}

fn save(fpath: &str, cred: &Credential) {
    match File::open(fpath) {
        Ok(f) => {
            let reader = BufReader::new(&f);
            for line in reader.lines() {
                let line = match line {
                    Ok(line) => line,
                    Err(_) => panic!("Error occured when trying to read content"),
                };
                if let Some(pos) = line.find('#') {
                    let username = &line[..pos];
                    if username == cred.username {
                        eprintln!("Username exists , try again!");
                        return;
                    }
                };
            }
        }
        Err(_) => {}
    }
    let mut f = OpenOptions::new()
        .append(true)
        .create(true)
        .open(fpath)
        .unwrap();

    let saved = format!("{}#{}#{}", cred.username, cred.salt, cred.hash);
    writeln!(f, "{}", saved).unwrap();
}

fn sign_up() -> Credential {
    let username = loop {
        print!("username:");
        io::stdout().flush().unwrap();
        let username: String = read_input();
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
        let password: String = read_input();
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
    let salt = gen_salt(5);
    let hashed = hash(&password, &salt);
    // println!("salt is {}", salt);
    //println!("hash is {}", hashed);
    let c = Credential {
        username: username,
        salt: salt,
        hash: hashed,
    };
    c
}

fn sign_in(fpath: &str, fail: &mut u32) {
    print!("username:");
    io::stdout().flush().unwrap();
    let username: String = read_input();
    print!("password:");
    io::stdout().flush().unwrap();
    let password: String = read_input();
    match File::open(fpath) {
        Ok(f) => {
            let reader = BufReader::new(&f);
            for line in reader.lines() {
                let line = match line {
                    Ok(line) => line,
                    Err(_) => panic!("Error when trying to read content"),
                };
                let result = verify_pass(&username, &password, &line);
                if result {
                    println!("Welcome home ! ");
                    return;
                }
            }
        }
        Err(_) => panic!("Error when trying to open the file"),
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

fn main() {
    let path = String::from("password.txt");
    let mut failed = 0;
    loop {
        println!();
        println!("╔════════════════════════════════════════╗");
        println!("║          AUTHENTICATION APP            ║");
        println!("╠════════════════════════════════════════╣");
        println!("║ 1) Sign Up                             ║");
        println!("║ 2) Sign In                             ║");
        println!("║ 3) Exit                                ║");
        println!("╚════════════════════════════════════════╝");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();
        let choice: u32 = read_input();
        if choice == 1 {
            println!("You have chosen the first option!");
            println!("---------------------------------SIGN UP---------------------------------");
            let credential = sign_up();
            save(&path, &credential);
        } else if choice == 2 {
            println!("You have chosen the second  option!");
            println!("---------------------------------SIGN IN---------------------------------");
            sign_in(&path, &mut failed);
        } else if choice == 3 {
            println!("You have chosen the third option!");
            println!("We are exiting.....");
            exit(0);
        } else {
            println!("Wrong option ! Try again");
        }
    }
}
