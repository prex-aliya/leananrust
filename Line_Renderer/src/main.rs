use std::env;

const PROGRAMENAME: &str = "2d line renderer";
const SSIZE: i16 = 32;

fn print_help() {
    println!("Usage: {} [--help/-t] [l/g] [num] [-i] [] [] [] []", PROGRAMENAME);
}
fn print_screen(pointx: i16, pointy: i16, point1x: i16, point1y: i16) {
    /* y = mx+b
     * find m slope
     */
    let x1: f32 = pointx.into();
    let y1: f32 = pointy.into();
    let x2: f32 = point1x.into();
    let y2: f32 = point1y.into();
    let slope: f32 = ( ((y1-y2)/(x1-x2)) ) as f32;
    println!("{}", slope);

    /* find y intercept or b */
    let intrecept = (slope*x1)-y1;
    println!("{}", intrecept);



    for y in 0..SSIZE {
        for x in 0..SSIZE {
            /* Transform current x and y into
             * float for the equation to work correctly.
             */
            let fx: f32 = x.into();
            let fy: f32 = y.into();

            /* sees if the x or y is equivilent to line */
            let line = ((slope*fx)-intrecept).round();

            if y == pointy && x == pointx {
                print!("..");
            } else if y == point1y && x == point1x {
                print!("..");
            } else if fy == line {
                print!(":)");
            } else {
                //print!("{:2}", y);
                print!("#@");
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

    print_screen(points[0], points[1], points[2], points[3]);
}
