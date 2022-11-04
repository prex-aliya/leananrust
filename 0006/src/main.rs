use std::env;

const PROGRAMENAME: &str = "numberline";

fn print_line() {

}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        if args[1] == "--help" {
            println!("Usage: {} [flags]", PROGRAMENAME);
            println!("  --help      display this help and exit");
            println!("  -n          no output to file");
            return;
        }
    }
    println!("this is after");
}
