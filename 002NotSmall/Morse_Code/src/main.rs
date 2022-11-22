/* SamMorse - Name after one of the inventers of telegraph,
 *  Samuel Morse, and what morse code is named after.
 *
 * This is a translator and encoder.
 */


fn translate(morse: u8) {
    println!("{}", morse);
}

fn main() {
    let morse: u8 = 0b10110101;
    translate(morse);
}

// vim: tw=80
