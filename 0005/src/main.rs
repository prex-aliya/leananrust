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

            let secret_rev = test_password(login, password);
            if secret_rev == true { break; }
        }
    } else {
        println!("Usage: accounts.ini");
        println!(" This is a Program That Shows");
        println!(" A Secret When Unlocked");
        println!(" Needs accounts.ini to be in present working directory");
    }
}

fn test_password(tlogin: String, tpass: String) -> bool {
    let conf = Ini::load_from_file("accounts.ini").unwrap();

    let user = conf.section(Some(tlogin)).unwrap();
    let password = user.get("password").unwrap();

    if password == tpass {
        let secret = user.get("secret").unwrap();
        println!("{}", secret);
        return true;
    } else {
        println!("[FAIL] Password is Incorrect!");
        return false;
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
