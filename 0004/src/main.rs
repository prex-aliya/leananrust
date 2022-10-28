extern crate rand;
use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();

    for _x in 0..5{
        let num: u8 = rng.gen_range(0..10);
        println!("Random number between 0 and 9: {}", num);
    }
}
