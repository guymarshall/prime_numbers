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

    number > 1 && is_prime && !is_not_prime
}

/*
AKS Test

(x - 1)^p - (x^p - 1)

If all coefficients are divisible by p then p is prime
*/

/*
Fermat's Little Theorem

a^p - a     ---->   is divisible by p
p is prime      1 <= a <= p

Check this for many values for a from 1 to p. The success rate for this test is over 99.999%.
*/
