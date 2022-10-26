use std::fs::File;
use std::io::Write;
use std::io;

fn main() {
    println!("\nWhat is your Name?");
    let name = get_input();

    let age = get_number_input();

    println!("\nWhat is your Username?");
    let user = get_input();


    println!("\n\nYour Name Is: {name}");
    println!("Your Age Is: {age}");
    println!("Your Username Is: {user}\n");

    println!("Do you want to save it to a file (Y/N)");
    let write_file = get_input();
    if write_file == "Y" {
        let output = format!("name:{} age:{} user:{}", name, age, user);
        println!("Exporting to file output.txt");
        let mut ofile = File::create("output.txt")
                       .expect("unable to create file");

        ofile.write_all(output.as_bytes()).expect("unable to write");
    }
    println!("Exiting");
}

fn get_number_input() -> u8 {
    loop {
        println!("\nWhat is your Age?");
        let answer = get_input();
        let _answer: u8 = match answer.trim().parse() {
            Ok(num) => {
                return num;
            },
            Err(_) => {
                continue
            },
        };
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
