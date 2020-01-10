fn multiples_of_3_and_5(number: u32) -> u32 {
    let mut sum = 0;
    for i in 1..number {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiples_of_3_and_5() {
        let nums = vec![10, 49, 1000, 19564, 8456];
        let expected = vec![23, 543, 233168, 89301183, 16687353];
        let i = nums.len() - 1;
        for j in 0..i {
            assert_eq!(multiples_of_3_and_5(nums[j]), expected[j]);
        }

    }
}

fn main() {
    println!("To run the test suite: cargo test");
}