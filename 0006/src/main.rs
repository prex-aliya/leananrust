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

fn print_line(points: &mut [i16; 4]) {
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

    let smallest: i16 = points[0];
    println!("Points Input: {:?}", points);

    //println!("{:-^1$}", "", 40-4);
    for _ in smallest..biggest {
        print!("---|");
    }
    print!("---|\n");


    //for i in smallest..biggest {
    for i in smallest..biggest+1 {
        if points[0] == i {
            print!("\x1b[31m{:4}\x1b[0m", i);
            while points[0] == i { /* remove zero */
                for i in 0..3 { points[i] = points[i+1]; }
            }
            points[3] = 0;
        } else {
            print!("{:4}", i);
        }
    }
    print!("\n");
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let mut points: [i16; 4] = [0, 0, 0, 0];

    if args.len() >= 2 {
        if args[1] == "--help" {
            println!("Usage: {} [--help/-n] [file]", PROGRAMENAME);
            println!("  --help      display this help and exit");
            println!("  -i          input for points");
            //println!("  -n          no output to file if empty");
            println!("input 0 for it to not to graph");
            return;
        } else if args[1] == "-i" {
            for i in 2..args.len() {
                let num: i16 = args[i].parse().unwrap();
                points[i-2] = num;
            }
            println!("{:?}",points);
        }
    } else {
        /* Get Points Input */
        points = [0, 1, 4, 6];
        for i in 0..4 {
            println!("Enter the {} number.", i+1);
            points[i] = get_number_input();
        }
    }


    /* Print Line */
    print_line(&mut points); /* unable to use zero */
}
