use std::env;

const PROGRAMENAME: &str = "m3t5";
const HOW_MUCH: i64 = 10;


fn print_help() {
    println!("Usage: {} [--help]", PROGRAMENAME);
    println!("  --help  Print this help message\n");
    println!("Red   == all multipals of 5");
    println!("blue  == all multipals of 3");
}

fn print_bar(
    how_much: i64, five: i8, three: i8, both: i8
) {
    for i in (0..how_much+1).rev() {
        print!(" #");

        if i <= both.into() {
            print!(" #");
        } else { print!("  "); }

        if i <= five.into() {
            print!(" #");
        } else { print!("  "); }

        if i <= three.into() {
            print!(" #");
        } else { print!("  "); }

        print!("\n");
    }
    println!("---------");
    println!("{} {} {} {}\n a b 5 3", how_much, both, five, three);
}

fn print_multiples(height: i8, how_much: i64) {
    let mut f: i8 = 0;
    let mut t: i8 = 0;

    let mut five: i8 = 0;
    let mut three: i8 = 0;
    let mut both: i8 = 0;


    for i in 0..how_much+1 {
        if f == 5 && t == 3 {
            t = 0;
            f = 0;
            print!("\x1b[33m{} \x1b[0m", i);
            both += 1;
        } else if f == 5 {
            f = 0;
            print!("\x1b[31m{} \x1b[0m", i);
            five += 1;
        } else if t == 3 {
            t = 0;
            print!("\x1b[34m{} \x1b[0m", i);
            three += 1;
        } else {
            print!("{} ", i);
        }
        f += 1;
        t += 1;
    }
    print!("\n");

    if height != 0 {
        print_bar(how_much, five, three, both);
    }

    return;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut how_much: i64 = HOW_MUCH;
    let mut height: i8 = 0;

    if args.len() >= 2 {
        if args[1] != "--help" {
            if args[1] == "--bar" {
                how_much = args[2].parse().unwrap();
                height = 10;
            } else {
                how_much = args[1].parse().unwrap();
            }
        } else {
            print!("{:?}", args);
            print_help();
            return;
        }
    } else {
            print_help();
            return;
    }

    print_multiples(height, how_much);

    return;
}
