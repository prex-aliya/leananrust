/* [SOURCES]
 * https://kerkour.com/rust-file-encryption-chacha20poly1305-argon2
 *
 */
extern crate ini;

use ini::Ini;
use std::{
    env,
    io,
};



fn main() {
    let args: Vec<String> = env::args().collect();
    let b = std::path::Path::new("accounts.ini").exists(); /* file exist */

    if b == true && args.len() == 1 {
        loop {
            println!("\nlogin: ");
            let login = get_input();

            println!("password: ");
            let password = get_input();

            let mut b = secret_reveal(login, password);
            if b == true {
                break;
            }

        }
    } else {
        println!("Usage: accounts.ini");
        println!(" This is a Program That Shows");
        println!(" A Secret When Unlocked");
        println!(" Needs accounts.ini to be in present working directory");
    }
}

fn secret_reveal(tlogin: String, tpass: String) -> bool {
    let conf = Ini::load_from_file("accounts.ini").unwrap();

    let user = conf.section(Some(tlogin)).unwrap();
    let username_truth: bool = test_user(tlogin);
    if username_truth == true {
        let password = user.get("password").unwrap();

        if password == tpass {
            println!("[PASS] Password is correct; revealing secret\n");
            let secret = user.get("secret").unwrap();
            println!("{}", secret);
            return true;
        } else {
            println!("[FAIL] Password is incorrect");
            return false;
        }
    } else {
        return false;
    }
}
fn test_user(try_user.as_ref().unwrap().path(): String) -> bool {
    let conf = Ini::load_from_file("accounts.ini").unwrap();

    let users = conf.section(Some("Users")).unwrap();
    let num = users.get("num").unwrap();
    let num_i: i32 = num
        .trim()
        .parse()
        .expect("Wanted a number");

    let mut username_truth: bool = false;

    for i in 0..num_i {
        let mut username = users.get(i.to_string()).unwrap();
        if try_user == username {
            println!("Username: {}", username);
            username_truth = true;
            break;
        } else {
            username_truth = false;
        }
    }
    return username_truth;
}

/* Streamlined Get Input*/
fn get_input() -> String {
    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read stdin");

    trim_newline(&mut answer);
    return answer;
}
fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}
