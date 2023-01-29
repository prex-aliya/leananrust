use std::collections::HashSet;



macro_rules! nothing_burger {
    () => {
        
    };
}

macro_rules! hello_ferris {
    () => {
        println!("Where's Ferris?");
    };
    ("Ferrir") => {
        println!("hello Ferris?");
    };
}

macro_rules! hello_x {
    ("Char") => {
        println!("Hello, Charlie");
    }
    /*($s:literal) => { println!("Hello, {}", $s); };
    ($s:ident) => { println!("Hello ident, {}", $s); };*/
    ($s:expr) => {
        println!("Hello ident, {}", $s);
    }
}

macro_rules! set {
    ($($s:expr),*) => {
        HashSet::from([$($s),*]);
    };
}

fn main() {
    nothing_burger!();
    hello_ferris!();
    hello_ferris!("Ferrir");
    
    hello_x!("Char");
    hello_x!("Buler");
    let a = "becv";
    hello_x!(a);

    let b = set!(1, 2, 3);
    dbg!(b);
}
