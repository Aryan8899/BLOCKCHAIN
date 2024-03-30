fn median(arr: &[i32]) -> f64 {
    let n = arr.len();
    if n == 0 {
        panic!("arr is empty");
    }
    if n % 2 == 1 {
        arr[n / 2] as f64
    } else {
        let mid = n / 2;
        (arr[mid - 1] as f64 + arr[mid] as f64) / 2.0
    }
}

fn main() {
    let arr_odd = [1, 2, 3, 4, 5];
    let arr_even = [1, 2, 3, 4];

    println!("Median of arr_odd: {}", median(&arr_odd));
    println!("Median of arr_even: {}", median(&arr_even));
}
