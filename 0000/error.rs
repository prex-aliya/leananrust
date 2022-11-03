fn main() {
    let first = "da3";
    let mut a:u32 = 0;
    match first.trim().parse() {
        Ok(val) => a = val,
        Err(_err) => {
            println!("Not a valid number!");
        }
    };
}
