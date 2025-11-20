mod authentication;
use crate::authentication::utils;
use std::{
    io::{self, Write},
    process::exit,
};
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
        let choice: u32 = authentication::utils::read_input();
        if choice == 1 {
            println!("You have chosen the first option!");
            println!("---------------------------------SIGN UP---------------------------------");
            let credential = authentication::auth::sign_up();
            utils::save(&path, &credential);
        } else if choice == 2 {
            println!("You have chosen the second  option!");
            println!("---------------------------------SIGN IN---------------------------------");
            authentication::auth::sign_in(&path, &mut failed);
        } else if choice == 3 {
            println!("You have chosen the third option!");
            println!("We are exiting.....");
            exit(0);
        } else {
            println!("Wrong option ! Try again");
        }
    }
}
