#![forbid(unsafe_code)]

mod math;

use promptput::input;

fn main() {
    let user_input: i32 = input("Enter a positive integer: ");

    let mut counter: i32 = 0;
    let mut prime_counter: i32 = 0;
    let mut primes: Vec<i32> = vec![];

    while prime_counter <= user_input {
        print!("\r{}", prime_counter);

        let is_prime: bool = math::is_prime_number(counter);

        if is_prime {
            prime_counter += 1;
            primes.push(counter);
        }

        counter += 1;
    }

    primes.iter().enumerate().for_each(|(index, prime)| {
        println!("{}: {}", index, prime);
    });
}
