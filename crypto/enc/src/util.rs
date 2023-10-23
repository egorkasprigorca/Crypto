use std::borrow::Cow;

use num::Zero;
use num_primes::BigUint;

extern crate num_bigint_dig as num_bigint;

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

pub fn num_inv_by_mod(num: &BigUint, module: &BigUint) -> BigUint {
    let res = num_bigint::algorithms::mod_inverse(
        Cow::Owned(num_bigint::BigUint::from_bytes_le(&*num.to_bytes_le())),
        Cow::Owned(num_bigint::BigUint::from_bytes_le(&*module.to_bytes_le()))
    ).unwrap();
    let res = num_primes::BigUint::from_bytes_le(&*res.to_bytes_le().1);
    res
}