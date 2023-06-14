#![forbid(unsafe_code)]

pub fn is_prime_number(number: i32) -> bool {
    let is_even: bool = (number & 1) == 0;
    let is_divisible: bool = (number % 3) == 0;

    let is_not_prime: bool = is_even || is_divisible;

    let mut counter: i32 = 5;
    let mut increment: i32 = 2;
    let mut is_prime: bool = true;

    while counter * counter <= number {
        let is_divisible: bool = (number % counter) == 0;
        is_prime &= !is_divisible;
        increment = 6 - increment;
        counter += increment;
    }

    return number > 1 && is_prime && !is_not_prime;
}
