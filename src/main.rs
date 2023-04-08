#![forbid(unsafe_code)]

mod math;
mod user_input;

fn main() {
    let user_input: u128 = user_input::get_user_input("Enter a positive integer: ");

    let mut counter: u128 = 0;
    let mut prime_counter: u128 = 0;
    let mut primes: Vec<u128> = vec![];

    while prime_counter <= user_input {
        let is_prime: bool = math::is_prime_number(counter);

        if is_prime {
            prime_counter += 1;
            primes.push(counter);
        }

        counter += 1;
    }
}
