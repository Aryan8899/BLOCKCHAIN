fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    for (index, &num) in arr.iter().enumerate() {
        if num == target {
            return Some(index);
        }
    }
    None
}

fn main() {
    let numbers = [5, 10, 15, 20, 25];
    let target_number = 15;

    match find_first_occurrence(&numbers, target_number) {
        Some(index) => println!(
            "The first occurrence of {} is at index {}",
            target_number, index
        ),
        None => println!("The number {} is not present in the array", target_number),
    }
}
