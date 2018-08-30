use std::io;

fn main() {
    println!("Enter the string to be Pig-latinized");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read from stdin");

    let len = input.len();
    input.truncate(len - 1);

    let first_char = input.chars().next().unwrap_or(' ');

    if first_char.is_ascii_alphabetic() {
        if "aeiouyAEIOUY".contains(first_char) {
            input.push_str("-hay");
        } else {
            input.remove(0);
            input.extend(&['-', first_char]);
            input.push_str("ay");
        }
    }
    println!("Your word pig-latinized is now: {}", input);
}
