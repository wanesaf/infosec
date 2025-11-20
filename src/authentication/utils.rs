use core::panic;
use rand::{rng, Rng};
use sha2::{Digest, Sha256};
use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    str::FromStr,
};

use crate::authentication::auth::Credential;

pub fn read_input<T: FromStr>() -> T {
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

pub fn gen_salt(len: usize) -> String {
    let mut r = rng();
    (0..len)
        .map(|_| r.random_range(0..10).to_string())
        .collect()
}

pub fn hash(password: &str, salt: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(format!("{}{}", password, salt));
    let r = hasher.finalize();
    format!("{:x}", r)
} // argon2 requires 8 bytes , we need only 5 bytes so we  must choose another algorithm , sha256 is not slow, maybe
  // iterations will be a good idea
  //
  //
  //
  //
pub fn verify_pass(usr: &str, pass: &str, cred: &str) -> bool {
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

pub fn save(fpath: &str, cred: &Credential) {
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
