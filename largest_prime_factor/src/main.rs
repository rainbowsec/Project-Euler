fn is_prime(n: u64) -> bool {
    let mut i = 3;
    let square_root_of_n = (n as f64).sqrt() as u64 + 1;
    if n <= 3 && n >= 1 {
        return true
    }
    while i < square_root_of_n {
        if n % i == 0 {
            return false
        }
        i += 2;
    }
    true
}

fn largest_prime_factor(n: u64) -> u64 {
    let mut prime_factor = n;
    let square_root_of_n = (n as f64).sqrt() as u64 + 1;
    for i in 2..square_root_of_n {
        if is_prime(i) {
            if n % i == 0 {
                prime_factor = i;
            }
        }
    }
    prime_factor
}

fn main() {
    println!("To run the test suite: cargo test");
}
