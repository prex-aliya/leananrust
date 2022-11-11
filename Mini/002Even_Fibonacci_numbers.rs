use std::env;

const PROGRAMENAME: &str = "efn";



fn print_help() {
    println!("Usage: {} [--help]", PROGRAMENAME);
    println!("  --help  Print this help message");
    println!("  --bar   print out bar graph, 0 is default");
    println!("              Default input is 4Million, by millions");
    println!("\nEven Fibonacci Numbers");
}

fn sequence(how_much: i32) -> [i8; 2] {
    let mut first: i32 = 1;
    let mut second: i32 = 1;
    let mut num_even: i8 = 0;
    let mut num: i8 = 0;
    let to_what: i32;

    if how_much == 0 {
        to_what = 4_000_000;
    } else { to_what = how_much*1_000_000; }

    println!("{}", to_what);
    print!("1 1 ");

    loop {
        let next = first + second;
        if next >= to_what {
            break
        }

        first = second;
        second = next;

        if next % 2 == 0 {
            num_even += 1;
            print!("\x1b[33m{} \x1b[0m", next);
        } else {
            print!("{} ", next);
        }

        num += 1;
    }

    println!("\n{}\ndone", num_even);
    let output: [i8; 2] = [num_even, num];
    return output;
}

fn print_bar(even: [i8; 2]) {
    let even_i: i32 = (((even[0]/even[1]) as f32)*10.0) as i32;

    println!("{}", even[1]);

    for i in (0..10+1).rev() {
        print!(" #");

        if i <= even_i.into() {
            print!(" #");
        } else { print!("  "); }

        print!("\n");
    }
    println!("---------");
    println!("{} {}\n a e", even[1], even[0]);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        if args[1] != "--help" {
            if args[1] == "--bar" {
                let _how_much: i32 = args[2].parse().unwrap();
                print_bar(sequence(_how_much));
            } else {
                let _how_much: i32 = args[1].parse().unwrap();
                sequence(_how_much);
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


    return;
}
