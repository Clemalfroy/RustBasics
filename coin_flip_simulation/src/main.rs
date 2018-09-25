extern crate rand;

use rand::{thread_rng, Rng};
use std::io;

fn user_input() -> Option<u32> {
    println!("How many times do you want to flip the coin ?");
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => Some(i),
        Err(..) => { eprintln!("Not a number"); None }
    }
}

fn main() {

    let n = match user_input() {
        Some(x) => x,
        None => return,
    };

    let mut tails = 0;
    let mut heads = 0;
    
    for _ in 0..n {
        if thread_rng().gen::<(bool)>() {
            tails += 1;
        } else {
            heads += 1;
        }
    }

    println!("Number of tails: {}, {}%", tails, (tails as f32 * 100.0) / n as f32);
    println!("Number of heads: {}, {}%", heads, (heads as f32 * 100.0) / n as f32);
}
