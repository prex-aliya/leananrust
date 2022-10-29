/* sources -
 * https://docs.rs/fastrand/latest/fastrand/
 *
 */
use fastrand;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 { /*if there are enough args. len includes zero */
        let amount: i16 = args[1].parse().unwrap(); /* get shift input from args*/
        let length: i16 = args[2].parse().unwrap(); /* get shift input from args*/

        for _i in 0..amount {
            let rand_str: String = std::iter::repeat_with(fastrand::alphanumeric)
                .take(length.try_into().unwrap())   /* length of string*/
                .collect();
            println!("{}", rand_str);
        }

    } else {
        println!("Usage: [AMOUNT] [LENGTH]");
        println!(" This is a Program That Prints Out Random Character");
        println!(" Passwords for Your Use.");
    }
}
