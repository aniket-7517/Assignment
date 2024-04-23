use std::io;

fn reverse_string_1(input: &str) -> String {
    input.chars().rev().collect()
}

fn main() {
    println!("Enter a string to reverse:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();

    let reversed_string_1 = reverse_string_1(input);
    println!("Reversed string : {}", reversed_string_1);
}
