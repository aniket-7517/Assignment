use std::io;

fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    
    let mut prefix = strings[0].clone();
    
    for string in strings.iter().skip(1) {
        while !string.starts_with(&prefix) {
            prefix.pop();
            if prefix.is_empty() {
                return String::new();
            }
        }
    }
    
    prefix
}

fn main() {
    println!("Enter a set of strings separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let strings: Vec<String> = input
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    println!("Longest common prefix: {}", longest_common_prefix(&strings));
}
