const HOW_MUCH: i64 = 10;

fn main() {
    let mut five: i8 = 0;
    let mut three: i8 = 0;

    for i in 0..HOW_MUCH {
        if five == 5 {
            five = 0;
            print!("{}", i);
        } else if three == 3 {
            three = 0;
            print!("{}", i);
        } else {
            print!("{}", i);
        }
    }
}
