use std::io;

fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    let (mut i, mut j) = (0, 0);

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            result.push(arr1[i]);
            i += 1;
        } else {
            result.push(arr2[j]);
            j += 1;
        }
    }

    result.extend_from_slice(&arr1[i..]);

    result.extend_from_slice(&arr2[j..]);

    result
}

fn main() {
    println!("Enter the elements of the first sorted array separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let arr1: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    println!("Enter the elements of the second sorted array separated by spaces:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let arr2: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    let merged = merge_sorted_arrays(&arr1, &arr2);
    println!("Merged array: {:?}", merged);
}
