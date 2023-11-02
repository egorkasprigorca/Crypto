use enc::util::{gcd, num_inv_by_mod};
use num::One;
use num_primes::{Generator, BigUint};

extern crate enc;

fn main() {
    let shared_p = Generator::new_prime(1024);
    let shared_p_neg1 = &shared_p - BigUint::one();
    let cA = loop {
        let cA = Generator::new_uint(1024);
        if gcd(&cA, &shared_p_neg1) == BigUint::one() {
            break cA;
        }
    };
    let dA = num_inv_by_mod(&cA, &(&shared_p - BigUint::one()));
    if ((&cA % &shared_p_neg1) * (&dA % &shared_p_neg1)) % (&shared_p - BigUint::one()) == BigUint::one() {
        println!("OKKKK");
    }

    let mut cards: Vec<BigUint> = Vec::with_capacity(52);
    for _ in 0..52 {
        let k = Generator::new_uint(1023);
        cards.push(k);
    }
}
