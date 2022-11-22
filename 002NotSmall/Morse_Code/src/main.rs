/* SamMorse - Name after one of the inventers of telegraph,
 *  Samuel Morse, and what morse code is named after.
 *
 * This is a translator and encoder.
 */


use std::env;

const PROGRAMENAME: &str = "SamMorse";


fn print_help() {
    println!("Usage: {} [--help/num] [num] [num] [num]", PROGRAMENAME);
}

fn translate() {
    let morse_key = vec![
       ".-".to_string(),
       "-...".to_string(),
       "-.-.".to_string(),
       "-..".to_string(),
       ".".to_string(),
       "..-.".to_string(),
       "--.".to_string(),
       "....".to_string(),
       "..".to_string(),
       ".---".to_string(),
       "-.-".to_string(),
       ".-..".to_string(),
       "--".to_string(),
       "-.".to_string(),
       "---".to_string(),
       ".--.".to_string(),
       "--.-".to_string(),
       ".-.".to_string(),
       "...".to_string(),
       "-".to_string(),
       "..-".to_string(),
       "...-".to_string(),
       ".--".to_string(),
       "-..-".to_string(),
       "-.--".to_string(),
       "--..".to_string()
    ];

    let letters = vec![
        "A".to_string(),
        "B".to_string(),
        "C".to_string(),
        "D".to_string(),
        "E".to_string(),
        "F".to_string(),
        "G".to_string(),
        "H".to_string(),
        "I".to_string(),
        "J".to_string(),
        "K".to_string(),
        "L".to_string(),
        "M".to_string(),
        "N".to_string(),
        "O".to_string(),
        "P".to_string(),
        "Q".to_string(),
        "R".to_string(),
        "S".to_string(),
        "T".to_string(),
        "U".to_string(),
        "V".to_string(),
        "W".to_string(),
        "X".to_string(),
        "Y".to_string(),
        "Z".to_string()
    ];
}


// TODO: Mak this a impl use name encode, this to the key or something
fn encode(input: char) {
    let morse_key = vec![
       ".-".to_string(),
       "-...".to_string(),
       "-.-.".to_string(),
       "-..".to_string(),
       ".".to_string(),
       "..-.".to_string(),
       "--.".to_string(),
       "....".to_string(),
       "..".to_string(),
       ".---".to_string(),
       "-.-".to_string(),
       ".-..".to_string(),
       "--".to_string(),
       "-.".to_string(),
       "---".to_string(),
       ".--.".to_string(),
       "--.-".to_string(),
       ".-.".to_string(),
       "...".to_string(),
       "-".to_string(),
       "..-".to_string(),
       "...-".to_string(),
       ".--".to_string(),
       "-..-".to_string(),
       "-.--".to_string(),
       "--..".to_string()
    ];

    let key: usize = ((input.to_ascii_uppercase() as i8) - 65).try_into().unwrap();

    print!("{}", morse_key[key]);
    //return morse_key[key];
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mode: u8 = 0b00000000;
    /* mode 0 == do nothing
     * mode 1 == decode
     * mode 2 == encode
     */

    /*
    for i in 0..args.len() {
        match args[i] {
            "-d" => { mode = 0b00000001
            },
            "-e" => { mode = 0b00000010
            },
            _ => { break; },
        }
    }
    */

    encode('H');
    encode('E');
    encode('L');
    encode('L');
    encode('O');
    encode('W');
    encode('O');
    encode('R');
    encode('L');
    encode('D');
    print!("\n");
}

// vim: tw=80
