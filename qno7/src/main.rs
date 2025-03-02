fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    if k > arr.len() {
        return None; // k is out of bounds
    }
    
    let mut sorted_arr = arr.to_vec(); // Create a mutable copy of the array
    sorted_arr.sort(); // Sort the array in ascending order
    
    Some(sorted_arr[k - 1]) // Return the kth smallest element
}

fn main() {
    let arr = [7, 10, 4, 3, 20, 15];
    let k = 3;

    match kth_smallest(&arr, k) {
        Some(kth) => println!("The {}th smallest element is: {}", k, kth),
        None => println!("Invalid value of k"),
    }
}
