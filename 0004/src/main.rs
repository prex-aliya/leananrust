use fastrand;

fn main() {
    for i in 1..12 {
        let rand_num = fastrand::i32(-23..43);
        println!("{}", rand_num);
    }
}
