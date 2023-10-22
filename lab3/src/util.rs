use num::Zero;
use num_primes::BigUint;

pub fn gcd(x: &BigUint, y: &BigUint) -> BigUint {
    let mut m = x.clone();
    let mut n = y.clone();
    while !m.is_zero() {
        let temp = m;
        m = n % &temp;
        n = temp;
    }
    n
}