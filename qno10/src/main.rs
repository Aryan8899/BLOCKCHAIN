fn prime(i: i32) {
    let mut prime = true;
    if i <= 1 {
        prime = false;
    } else {
        for j in 2..=i / 2 {
            if i % j == 0 {
                prime = false;
                break;
            }
        }
    }

    if prime {
        println!("{} is a prime number", i);
    } else {
        println!("{} not a prime number", i);
    }
}

fn main() {
    prime(5);
}
