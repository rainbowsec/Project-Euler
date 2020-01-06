fn multiples_of_3_and_5(number: u32) -> u32 {
    let mut sum = 0;
    for i in 1..number {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    sum
}

fn main() {
    let nums = vec![10, 49, 1000, 19564, 8456];
    for num in nums {
        println!("The sum of all integers divisible by 3 or 5 under {} is {}", num, multiples_of_3_and_5(num));
    }
}
