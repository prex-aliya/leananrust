extern crate ini;
use ini::Ini;
use std::env;
use std::io;


fn main() {
    let args: Vec<String> = env::args().collect();
    let b = std::path::Path::new("accounts.ini").exists(); /* file exist */
    //let output = format!("name:{} age:{} user:{}", name, age, user);

    if b == true {
        println!("\nlogin: ");
        let login = get_input();

        println!("password: ");
        let password = get_input();

        try_password(login, password);
    } else {
        println!("Usage: [FILE]");
        println!(" This is a Program That Shows");
        println!(" A Secret When Unlocked");
    }
}

fn try_password(tlogin: String, tpass: String) {
    let conf = Ini::load_from_file("accounts.ini").unwrap();

    let user = conf.section(Some(tlogin)).unwrap();
    let password = user.get("password").unwrap();

    if password == tpass {
        println!("Password is Correct\nReveling Secret");
        let secret = user.get("secret").unwrap();
        println!("\n\n{}\n\n", secret);
    }
}

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
