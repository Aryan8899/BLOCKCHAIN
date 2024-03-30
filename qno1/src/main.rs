fn main() {
    //calling
    let result = palindromo("level");
    println!("Is it a palindrome? {}", result);
}

fn palindromo(name: &str) -> bool {
    for i in 0..name.len() / 2 {
        if name.as_bytes()[i] != name.as_bytes()[name.len() - i - 1] {
            return false;
        }
    }
    true
}
