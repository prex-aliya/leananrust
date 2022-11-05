use std::env;

const PROGRAMENAME: &str = "numberline";



fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}
fn get_input() -> String {
    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read stdin");
    trim_newline(&mut answer);
    return answer;
}

fn print_line() {

}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        if args[1] == "--help" {
            println!("Usage: {} [--help/-n] [file]", PROGRAMENAME);
            println!("  --help      display this help and exit");
            println!("  -n          no output to file if empty");
            return;
        }
    }

    /* probably cant put in
    {{{
    savelocation = arg[2] if it exists

    if
        savelocation is empty,
        then do nothing skip
    else if
        savelocation is not empty
        then call write_to_dir( filename/dir, what to write )
    else
        ask if
            want to saver
            askes takes input and parses
            puts into savelocation
        else
            do nothing
    done
    }}}
     */

    print_line();
}
