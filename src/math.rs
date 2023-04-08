#![forbid(unsafe_code)]

pub fn is_prime_number(number: i32) -> bool {
    if number % 2 == 0 {
        return false;
    }
    let mut counter: i32 = 3;
    while counter < number {
        if number % counter == 0 {
            return false;
        }
        counter += 2;
    }
    return true;
}