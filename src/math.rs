#![forbid(unsafe_code)]

pub fn is_prime_number(number: i32) -> bool {
    let is_even = (number & 1) == 0;
    let is_divisible = (number % 3) == 0;

    let is_not_prime = is_even || is_divisible;

    let mut counter = 5;
    let mut increment = 2;
    let mut is_prime = true;

    while counter * counter <= number {
        let is_divisible = (number % counter) == 0;
        is_prime &= !is_divisible;
        increment = 6 - increment;
        counter += increment;
    }

    return number > 1 && is_prime && !is_not_prime;
}
