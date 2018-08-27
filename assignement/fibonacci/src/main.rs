fn main() {
    println!("Value: {}", fibonacci(14)); 
}

fn fibonacci(nb : u32) -> u32 {
    if nb == 0 {
        0
    } else if nb == 1 {
        1
    } else {
        fibonacci(nb - 1) + fibonacci(nb - 2)
    }
}