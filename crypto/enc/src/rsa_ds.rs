use std::{fs, path::Path};

use num::One;
use num_primes::{BigUint, Generator};
use sha2::{Digest, Sha512};

use crate::util::{gcd, num_inv_by_mod};

pub fn init(bits: usize) -> (BigUint, BigUint, BigUint, BigUint, BigUint) {
    let p = Generator::new_prime(bits);
    let q = Generator::new_prime(bits);
    let n = &p * &q;

    let fi = (&p - BigUint::one()) * (&q - BigUint::one());

    let d = loop {
        let d = Generator::new_uint(bits);
        if gcd(&fi, &d) == BigUint::one() && &d < &fi {
            break d;
        }
    };

    let c = num_inv_by_mod(&d, &fi);

    (c, d, n, p, q)
}

pub fn init_many_c_d(bits: usize, num_cd: usize) -> (Vec<BigUint>, Vec<BigUint>, BigUint, BigUint, BigUint) {
    let p = Generator::new_prime(bits);
    let q = Generator::new_prime(bits);
    let n = &p * &q;

    let fi = (&p - BigUint::one()) * (&q - BigUint::one());

    let mut vec_c = Vec::with_capacity(num_cd);
    let mut vec_d = Vec::with_capacity(num_cd);

    for _ in 1..num_cd {
        let c = loop {
            let c = Generator::new_uint(bits);
            if gcd(&fi, &c) == BigUint::one() {
                break c;
            }
        };

        let d = num_inv_by_mod(&c, &fi);
        vec_c.push(c);
        vec_d.push(d);
    }

    (vec_c, vec_d, p, q, n)
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
