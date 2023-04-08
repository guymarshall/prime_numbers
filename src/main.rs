#![forbid(unsafe_code)]

mod math;
mod user_input;

fn main() {
    let user_input: i32 = user_input::input("Enter a positive integer: ");

    let mut counter: i32 = 0;
    let mut prime_counter: i32 = 0;
    let mut primes: Vec<i32> = vec![];

    while prime_counter <= user_input {
        let is_prime: bool = math::is_prime_number(counter);

        if is_prime {
            prime_counter += 1;
            primes.push(counter);
        }

        counter += 1;
    }

    primes.into_iter().for_each(|prime| println!("counter: {}", prime));
}
