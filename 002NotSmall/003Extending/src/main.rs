mod extensions {
    pub fn begin() {
        /* calles external list */
        println!("begin!");
    }

    pub fn move_left() {
        println!("moved left!");
    }

    pub fn end() {
        /* calles external list */
        println!("end!\n");
    }
}



fn main() {
    println!("Hello, world!");

    loop {


        extensions::begin();

        extensions::move_left();

        extensions::end();
    }
}
