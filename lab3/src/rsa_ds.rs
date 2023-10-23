use std::{borrow::Cow, path::Path, fs};

use num::One;
use num_primes::{Generator, BigUint};
use sha2::{Digest, Sha512};

use crate::util::{gcd, num_inv_by_mod};

extern crate num_bigint_dig as num_bigint;

pub fn init(bits: usize) -> (BigUint, BigUint, BigUint) {
    let p = Generator::new_prime(bits);
    let q = Generator::new_prime(bits);
    let n = &p * &q;

    let fi = (&p - BigUint::one()) * (&q - BigUint::one());

    let d = loop {
        let d = Generator::new_uint(bits);
        if gcd(&fi, &d) == BigUint::one() {
            break d;
        }
    };

    // let c = num_bigint::algorithms::mod_inverse(
    //     Cow::Owned(num_bigint::BigUint::from_bytes_le(&*d.to_bytes_le())),
    //     Cow::Owned(num_bigint::BigUint::from_bytes_le(&*fi.to_bytes_le()))
    // ).unwrap();
    // let c = num_primes::BigUint::from_bytes_le(&*c.to_bytes_le().1);
    let c = num_inv_by_mod(&d, &fi);

    (c, d, n)
}

fn sign(message: &[u8], c: &BigUint, n: &BigUint) -> BigUint {
    let mut hasher = Sha512::new();
    hasher.update(message);
    let y = hasher.finalize();
    let y = num_primes::BigUint::from_bytes_le(y.as_slice());
    y.modpow(&c, &n)
}

fn verify(message: &[u8], s: &BigUint, d: &BigUint, n: &BigUint) {
    let mut hasher = Sha512::new();
    hasher.update(message);
    let h = hasher.finalize();
    let h = num_primes::BigUint::from_bytes_le(h.as_slice());
    let w = s.modpow(d, n);
    assert_eq!(h, w);
}

pub fn sign_file<P: AsRef<Path>>(path: P, sign_file_path: P, c: &BigUint, n: &BigUint) {
    let binding = fs::read(path).unwrap();
    let msg_bytes = binding.as_slice();
    let s = sign(msg_bytes, c, n);
    fs::File::create(&sign_file_path).unwrap();
    fs::write(sign_file_path, s.to_bytes_le()).unwrap();
}

pub fn verify_file<P: AsRef<Path>>(path: P, sign_file_path: P, d: &BigUint, n: &BigUint) {
    let binding = fs::read(path).unwrap();
    let msg_bytes = binding.as_slice();
    let s = fs::read(sign_file_path).unwrap();
    let s = num_primes::BigUint::from_bytes_le(s.as_slice());
    verify(msg_bytes, &s, d, n);
}