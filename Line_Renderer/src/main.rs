use std::env;

const PROGRAMENAME: &str = "2d line renderer";
const SSIZE: i16 = 40;

fn print_help() {
    println!("Usage: {} [--help/num] [num] [num] [num]", PROGRAMENAME);
}

fn calc(points: [i16; 4]) -> [f32; 2] {
    /* points to float for a more complete */
    let fx: f32 = points[0].into();
    let fy: f32 = points[1].into();
    let fx1: f32 = points[2].into();
    let fy1: f32 = points[3].into();

    /* y = mx+b
     * find, m, slope
     */
    let m: f32 = ((fy1-fy)/(fx1-fx)).into();

    /* find y intercept or b */
    let b: f32 = ((m*fx)-fy).into();

    let output: [f32; 2] = [m, b];
    return output;
}
fn calc_line(x: f32, m: f32, b: f32) -> f32 {
    let line: f32 = (m*x)-b;

    return line;
}
fn print_screen(points: [i16; 4]) {
    /* points to float */
    let fx: f32 = points[0].into();
    let fy: f32 = points[1].into();
    let fx1: f32 = points[2].into();
    let fy1: f32 = points[3].into();

    /* Print Inputs and Calculated */
    let calc: [f32; 2] = calc(points);
    let m: f32 = calc[0];
    let b: f32 = calc[1];
    println!("Slope, y-intercept: {:?}", calc);

    for y in (0..SSIZE).rev() {
        for x in 0..SSIZE {

            let fx: f32 = x.into();
            let fy: f32 = y.into();
            let line: f32 = calc_line(fx, m, b);

            if x == points[0] && y == points[1]
                || x == points[2] && y == points[3]
            {
                //print!("..");
                print!("..");
            } else if fy == line {
                print!(":]");
            } else {
                print!("#@");
                //print!("{} ", line.round());
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

    //points = [2, 2, 8, 8];
    print_screen(points);
}
