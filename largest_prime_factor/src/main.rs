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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_prime_factor() {
        let nums = vec![2, 3, 5, 7, 13195, 600851475143];
        let expected = vec![2, 3, 5, 7, 29, 6857];
        let i = nums.len() - 1;
        for j in 0..i {
            assert_eq!(largest_prime_factor(nums[j]), expected[j]);
        }

    }

    #[test]
    fn test_is_prime() {
        let nums = vec![2, 3, 5, 7, 29, 6857];
        for num in nums {
            assert!(is_prime(num));
        }
    }
}

fn main() {
    println!("To run the test suite: cargo test");
}
