use std::env;

const PROGRAMENAME: &str = "2d line renderer";
const SSIZE: i16 = 16;

fn print_help() {
    println!("Usage: {} [--help/-t] [l/g] [num] [-i] [] [] [] []", PROGRAMENAME);
}
fn print_screen(points: [i16; 4]) {
    /* y = mx+b
     * find m slope
     */
    let x1: f32 = points[0].into();
    let y1: f32 = points[1].into();
    let x2: f32 = points[2].into();
    let y2: f32 = points[3].into();
    let slope: f32 = ( ((y1-y2)/(x1-x2)) ) as f32;
    println!("Slope {}", slope);

    /* find y intercept or b */
    let intrecept = (slope*x1)-y1;
    println!("Intercept {}", intrecept);



    for y in (0..SSIZE).rev() {
        for x in 0..SSIZE {
            /* Transform current x and y into
             * float for the equation to work correctly.
             */
            let fx: f32 = x.into();
            let fy: f32 = y.into();

            /* sees if the x or y is equivilent to line */
            let liney: i16 = ((slope*fx)-intrecept).round() as i16;

            if y == points[1] && x == points[0] {
                print!("..");
            } else if y == points[3] && x == points[2] {
                print!("..");
            } else if y == liney {
                print!("\x1b[36m:)\x1b[0m");
            //} else if fy == liney.round() || fy == liney.ceil()+0.5 {
                //print!("[:");
            } else {
                print!("{:2} ", liney);
                //print!("#@");
            }
        }
        print!("\n");
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
