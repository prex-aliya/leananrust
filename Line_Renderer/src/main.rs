use std::env;

const PROGRAMENAME: &str = "2d line renderer";
const SSIZE: i16 = 16;

fn print_help() {
    println!("Usage: {} [--help/num] [num] [num] [num]", PROGRAMENAME);
}
fn calc(points: [i16; 4]) -> [f32; 2] {
    /* y = mx+b
     * find, m, slope
     */
    let m: f32 = ((points[3]-points[1])/(points[2]-points[0])).into();

    /* make x and y a float */
    let fx: f32 = points[0].into();
    let fy: f32 = points[1].into();

    /* find y intercept or b */
    let b: f32 = ((m*fx)-fy).into();

    let output: [f32; 2] = [m, b];
    return output;
}
fn print_screen(points: [i16; 4]) {

    /* Print Inputs and Calculated */
    let calc: [f32; 2] = calc(points);
    println!("Slope, y-intercept: {:?}", calc);


    for y in (0..SSIZE).rev() {
        for x in 0..SSIZE {
            let fy: f32 = y.into(); /* convert i16 to f32 for capability */
            //let fx: f32 = y.into();
            let line: i16 = (((fy-calc[1])/calc[0])).round() as i16;
            //let liney: i16 = (((calc[0]*fx)+calc[1])).round() as i16;

            if x == points[0] && y == points[1]
                || x == points[2] && y == points[3]
            {
                print!("..");
            } else if x == line-2 {
                print!(":]");
            } else {
                print!("#@");
                //print!("{}", line);
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
