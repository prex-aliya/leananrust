use std::io;

fn main() {
    println!("FMA Physics Calculator\n
        If your finding that Variable put it a 0\n");

    loop {
        /* Get Variables In */
        println!("\nInput Variable Force (F)");
        let force = get_number_input();
        println!("\nInput Variable Mass (M)");
        let mass = get_number_input();
        println!("\nInput Variable Acceleration (A)");
        let acceleration = get_number_input();

        println!("The Variables You Put In are F:{} M:{} A:{}",
            force, mass, acceleration);
        println!("Do You Want To Retry (y/n)?");

        let mut retry = get_input();
        if retry == "y" || retry == "Y" {
            println!("Redoing Variables");
        } else {
            loop {
                println!("Pick From One of The Three Options");
                println!("(1) : F = M*A");
                println!("(2) : M = F/A");
                println!("(3) : A = F/M");
                println!("(q) : quit");
                let mut retry = get_input();

                if retry == "1" {
                    println!("F = M*A: {}", mass * acceleration);
                    break;
                } else if retry == "2" {
                    println!("M = F/A: {}", force/acceleration);
                    break;
                } else if retry == "3" {
                    println!("A = F/M: {}", force/mass);
                    break;
                } else if retry == "q" {
                    break;
                } else {
                    println!("error: not an available option");
                }
            }
            break;
        }
    }
}


fn get_number_input() -> f64 {
    loop {
        let answer = get_input();
        let _answer: f64 = match answer.trim().parse() {
            Ok(num) => { return num; },
            Err(_) => { continue },
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
