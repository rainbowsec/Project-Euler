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

fn main() {
    let nums = vec![10, 18, 23, 43];
    for num in nums {
        println!("The sum of all first {} even term of fibonnaci is {}", num, fibo_even_sum(num));
    }
}