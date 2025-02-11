#![forbid(unsafe_code)]

pub fn is_prime_number(number: i32) -> bool {
    let mut counter: i32 = 2;

    while counter * counter <= number {
        if number % counter == 0 {
            return false;
        }

        counter += 1;
    }

    true
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
