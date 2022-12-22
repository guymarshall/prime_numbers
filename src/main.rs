mod math;
mod user_input;

fn main() {
    let user_input: u128 = user_input::get_user_input("Enter a positive integer: ");

    let is_prime: bool = math::is_prime_number(user_input);
    
    println!("{} is prime: {}", user_input, is_prime);
}
