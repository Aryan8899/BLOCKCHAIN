fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let mut prefix = strings[0].clone();

    for s in strings.iter().skip(1) {
        // Adjust the prefix length to the length of the shortest string
        let mut i = 0;
        while i < prefix.len() && i < s.len() && prefix.chars().nth(i) == s.chars().nth(i) {
            i += 1;
        }
        prefix = prefix[..i].to_string(); // Extract the common prefix
        if prefix.is_empty() {
            break;
        }
    }

    prefix
}

fn main() {
    let strings = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];
    let common_prefix = longest_common_prefix(&strings);
    println!("Longest common prefix: {}", common_prefix);
}
