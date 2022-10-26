/*
 * Caesar Cipher
 * r/dailyprogramer
 * https://www.reddit.com/r/dailyprogrammer/comments/pkw2m/2112012_challenge_3_easy/
 *
 * PROMPT:
 * write a program that can encrypt texts with an alphabetical caesar cipher.
 * This cipher can ignore numbers, symbols, and whitespace.
 *
 * for extra credit, add a "decrypt" function to your program!
 *
 */

use std::env;


const ASCII_A: i8 = 'A' as i8;
const ASCII_Z: i8 = 'Z' as i8;
const ABC_SIZE: i8 = ASCII_Z - ASCII_A + 1;


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut outputtext = String::new();

    println!("Arguments : {}", args.len() - 1); /* Length of Array */

    if args.len()-1 > 1 {
        let shift: i8 = args[1].parse().unwrap(); /* get shift input from args*/
        println!("{:?}", &args[2..]);

        for i in 2..args.len() {
            let input = args[i].to_string();
            let char_vec: Vec<char> = input.chars().collect();
            for c in char_vec {
                outputtext.push(char_shift(c, shift));
            }
            outputtext.push(' ');
        }

        println!("{outputtext}");
    } else {
        println!("Usage: [SHIFT]... [STRING]...");
    }
}

fn char_shift(input: char, shift: i8) -> char {
    // --- guard statements
    if !input.is_ascii_alphabetic() {
        return input;
    }

    if shift >= ABC_SIZE || shift <= -ABC_SIZE {
       panic!("invalid shift");
    }

    // --- let's focus on uppercase only
    let input_upper = input.to_ascii_uppercase();

    // --- char to index within alphabet
    let input_index = input_upper as i8 - ASCII_A;

    // --- shift
    let output_index = (input_index + shift).rem_euclid(ABC_SIZE);

    // --- index to char
    let output_upper = ((output_index + ASCII_A) as u8) as char;

    // --- return, possibly after converting to lowercase
    if input.is_ascii_lowercase() {
        output_upper.to_ascii_lowercase();
    }
    return output_upper;
}
