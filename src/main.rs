#![forbid(unsafe_code)]

mod math;

use promptput::input;

fn main() {
    let user_input: i32 = input("Enter a positive integer: ");

    let mut number: i32 = 2;
    let mut prime_count: i32 = 0;
    let mut primes: Vec<i32> = vec![];

    while prime_count < user_input {
        let is_prime: bool = math::is_prime_number(number);

        if is_prime {
            prime_count += 1;
            primes.push(number);
        }

        number += 1;
    }

    primes.iter().enumerate().for_each(|(index, prime)| {
        println!("{}: {}", index + 1, prime);
    });
}
