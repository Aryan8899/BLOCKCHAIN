fn reverse(str: &str) -> String {
    let mut reverse = String::new();

    for c in str.chars().rev() {
        reverse.push(c);
    }
    reverse
}

fn main() {
    let original = "Hello, world!";
    let reversed = reverse(original);
    println!("Original: {}", original);
    println!("Reversed: {}", reversed);
}
