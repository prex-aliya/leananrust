use std::env;

const PROGRAMENAME: &str = "2d line renderer";
const SSIZE: i16 = 16;

fn print_help() {
    println!("Usage: {} [--help/-t] [l/g] [num] [-i] [] [] [] []", PROGRAMENAME);
}
fn print_screen(points: [i16; 4]) {
    /* y = mx+b                        //
     * find m slope                    // Renovation Trying to
     */                                // Plot it out first,
                                       // as to got get twisted up.
    /* find y intercept or b */        //


    for y in (0..SSIZE).rev() {
        for x in 0..SSIZE {
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut points: [i16; 4] = [0, 0, 0, 0];

    if args.len() >= 2 {
        if args[1] == "--help" {
            print_help();
            return;
        } else {
            if args.len() >= 3 {
                let mut a = 0;
                for i in 1..args.len() {
                    let num: i16 = args[i].parse().unwrap();
                    points[a] = num;
                    a += 1;
                }
            }
        }
    } else {
        print_help();
        return;
    }

    print_screen(points);
}
