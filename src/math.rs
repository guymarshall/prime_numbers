pub fn is_prime_number(number: u128) -> bool {
    if number % 2 == 0 {
        return false;
    }
    let mut counter: u128 = 3;
    while counter < number {
        if number % counter == 0 {
            return false;
        }
        counter += 2;
    }
    return true;
}