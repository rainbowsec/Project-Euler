fn fibo_even_sum(max: u32) -> u32 {
    let mut numbers = vec![0, 2];
    let mut sum = 2;
    let max = (max + max % 2) / 3;
    for _i in 1..max {
        let next_term = 4 * numbers[1] + numbers[0];
        numbers[0] = numbers[1];
        numbers[1] = next_term;
        sum += numbers[1];
    }
    sum
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibo_even_sum() {
        let nums = vec![10, 18, 23, 43];
        let expected = vec![44, 3382, 60696, 350704366];
        let i = nums.len() - 1;
        for j in 0..i {
            assert_eq!(fibo_even_sum(nums[j]), expected[j]);
        }

    }
}

fn main() {
    println!("To run the test suite: cargo test");
}