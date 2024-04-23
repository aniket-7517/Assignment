use std::io;

fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;
    let mut result = None;

    while low <= high {
        let mid = low + (high - low) / 2;
        
        if arr[mid] == target {
            result = Some(mid);
            high = mid - 1;
        } else if arr[mid] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    result
}

fn main() {
    println!("Enter the sorted array elements separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let arr: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    println!("Enter the number to search:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let target: i32 = input.trim().parse().expect("Invalid input");

    match find_first_occurrence(&arr, target) {
        Some(index) => println!("The first occurrence of {} is at index {}", target, index),
        None => println!("{} not found in the array", target),
    }
}
