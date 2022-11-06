use std::{ env, io, };

const PROGRAMENAME: &str = "numberline";



fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
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
fn get_number_input() -> i16 {
    loop {
        let input = get_input();
        let input: i16 = match input.trim().parse() {
            Ok(num) => {
                return num;
            },
            Err(_) => {
                continue
            },
        };
    }
}

fn print_line(points: &mut [i16; 4], greater: bool, lesser: bool) {
    let mediator = points[0];

    points.sort(); /* Sort from smallest to biggest */
    let biggest: i16 = points[3];
    //println!("{}", points[3]);

    while points[0] == 0 { /* remove zero */
        for i in 0..3 {
            points[i] = points[i+1];
            //println!("{}{:?}", i, points);
        }
        points[3] = 0;
    }

    println!("Points Input: {:?}", points); /* Tells User
                                            There Points */
    let begin: i16;
    let endin: i16;
    if points[0] >= -10 && biggest <= 10 {
        begin = -10;
        endin = 10;
    } else if points[0] >= 1 && biggest < 20 {
        begin = 0;
        endin = 20;
    } else {
        begin = points[0];
        endin = biggest;
    }
    /*
     * unable to do negative -20 to 0
     *  if biggest and smallest are in the
     *  negatives :(
     */

    for _ in begin..endin {
        print!("---|");
    }
    print!("---|\n");


    //for i in smallest..biggest {
    // changed to stop adding more not needed variables
    for i in begin..endin+1 { /* +1 for for loop */
        if points[0] == i {
            print!("\x1b[31m{:4}\x1b[0m", i);
            while points[0] == i { /* remove zero */
                for i in 0..3 { points[i] = points[i+1]; }
            }
            points[3] = 0;
        } else if greater == true && mediator <= i {
            print!("\x1b[34m{:4}\x1b[0m", i);
        } else if lesser == true && mediator >= i {
            print!("\x1b[34m{:4}\x1b[0m", i);
        } else {
            print!("{:4}", i);
        }
    }
    print!("\n");
}
fn print_help() {
    println!("Usage: {} [--help/-n] []..", PROGRAMENAME);
    println!("  --help              display this help and exit");
    println!("  -i  [num] [] [] []  input for points");
    println!("  -t  [g/l] [num]     for to a greater or to a lesser like: x>4");
    //println!("  -n          no output to file if empty");
    println!("input 0 for it to not to graph");
    println!(" 0 for it to not to graph.");
    println!("use only one at time.");
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut points: [i16; 4] = [0, 0, 0, 0];
    let mut greater: bool = false;
    let mut lesser: bool = false;
    let mut points_input = 4;
    let mut threw_input = 1;

    for i in 0..4 {
        println!("{}", i);
        if args[i] == "-i" {
            points_input = 3;
        }
        if args[i] == "-i" {
            threw_input = i;
        }
    }

    print!("{}", points_input);

    if args[1] == "--help" {
        print_help();
        return;
    }
    if args[points_input] == "-i" {
        let mut a = 0;
        for i in points_input+1..args.len() {
            let num: i16 = args[i].parse().unwrap();
            points[a] = num;
            println!("{}", i);
            a = a+1;
        }
        println!("{:?}",points);
    }
    if args[threw_input] == "-t" { /* for through functions */
            let f = threw_input + 1;
            let t = threw_input + 2;

            if args[f].to_lowercase() == "greater" ||
            args[f].to_lowercase() == "great" ||
            args[f].to_lowercase() == "g" {
                greater = true;
            } else if args[f].to_lowercase() == "lesser" ||
            args[f].to_lowercase() == "less" ||
            args[f].to_lowercase() == "l" {
                lesser = true;
            }
            points[0] = args[t].parse().unwrap();
    }
    if args.len() >= 2 {
    } else {
        /* Get Points Input */
        points = [0, 1, 4, 6];
        for i in 0..4 {
            println!("Enter the {} number.", i+1);
            points[i] = get_number_input();
        }
    }

    //points = [12, 2 , 5, 7];

    /* Print Line */
    print_line(&mut points, greater, lesser); /* unable to use zero */
}
