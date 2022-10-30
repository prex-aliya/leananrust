/* [SOURCES]
 * https://kerkour.com/rust-file-encryption-chacha20poly1305-argon2
 *
 */
extern crate ini;
extern crate argon2;

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use ini::Ini;
use rand::{rngs::OsRng, RngCore};
use std::{
    io,
    env,
    fs::File,
};



fn main() {
    let args: Vec<String> = env::args().collect();
    let b = std::path::Path::new("accounts.ini").exists(); /* file exist */

    if b == true {
        println!("\nlogin: ");
        let login = get_input();

        println!("password: ");
        let password = get_input();

        try_password(login, password);
    } else {
        println!("Usage: accounts.ini");
        println!(" This is a Program That Shows");
        println!(" A Secret When Unlocked");
        println!(" Needs accounts.ini to be in present working directory");
    }
}

fn try_password(tlogin: String, tpass: String) {
    let conf = Ini::load_from_file("accounts.ini").unwrap();

    let user = conf.section(Some(tlogin)).unwrap();
    let password = user.get("password").unwrap();

    if password == tpass {
        let secret = user.get("secret").unwrap();
        let b = std::path::Path::new(secret).exists();
        /* file exist */
        println!("{}", b);
    }
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
