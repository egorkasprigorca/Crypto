use std::{borrow::Cow, path::Path, fs, io::{Write, self, Read, BufRead, BufReader}};

use num::One;
use num_primes::{Generator, BigUint};
use sha2::{Digest, Sha256, Sha512};

use crate::util::gcd;

extern crate num_bigint_dig as num_bigint;

pub fn init(bits: usize) -> (BigUint, BigUint, BigUint, BigUint) {
    let p = Generator::new_prime(bits);
    let g = Generator::new_prime(bits);

    let mut x;
    loop {
        // Alice's private key
        x = Generator::new_uint(bits);
        if x > BigUint::one() && x < &p - BigUint::one() {
            break;
        }
    }

    // Alice's public key
    let y = g.modpow(&x, &p);
    (p, g, x, y)
}

fn sign(message: &[u8], p: &BigUint, g: &BigUint, x: &BigUint) -> (BigUint, BigUint) {
    let mut hasher = Sha512::new();
    hasher.update(message);
    let h = hasher.finalize();
    let h = num_primes::BigUint::from_bytes_le(h.as_slice());

    let mut k;
    loop {
        k = Generator::new_uint(1024);
        if k < p - BigUint::one() && gcd(&k, &(p - BigUint::one())) == BigUint::one() {
            break;
        }
    }

    let r = g.modpow(&k, &p);

    let b = x*&r - &h;
    let u = (p - BigUint::one()) - (b % (p - BigUint::one()));

    let k_inv = num_bigint::algorithms::mod_inverse(
        Cow::Owned(num_bigint::BigUint::from_bytes_le(&*k.to_bytes_le())),
        Cow::Owned(num_bigint::BigUint::from_bytes_le(&*(p - BigUint::one()).to_bytes_le()))
    ).unwrap();
    let k_inv = num_primes::BigUint::from_bytes_le(&*k_inv.to_bytes_le().1);
    let s = &k_inv * &u % (p - BigUint::one());
    (r, s)
}

fn verify(message: &[u8], y: &BigUint, r: &BigUint, s: &BigUint, g: &BigUint, p: &BigUint) {
    let mut hasher = Sha512::new();
    hasher.update(message);
    let h = hasher.finalize();
    let h = num_primes::BigUint::from_bytes_le(h.as_slice());
    let left = ((y.modpow(r, p)) * (r.modpow(s, p))) % p;
    let right = g.modpow(&h, p);
    assert_eq!(left, right);
}

pub fn sign_file<P: AsRef<Path>>(path: P, sign_file_path: P, p: &BigUint, g: &BigUint, x: &BigUint) {
    let binding = fs::read(path).unwrap();
    let msg_bytes = binding.as_slice();
    let (r, s) = sign(msg_bytes, p, g, x);
    let r = r.to_bytes_le();
    let s = s.to_bytes_le();
    let mut sign_file = fs::File::create(sign_file_path).unwrap();
    sign_file.write_all(r.len().to_le_bytes().as_slice()).unwrap();
    sign_file.write_all(r.as_slice()).unwrap();
    sign_file.write_all(s.len().to_le_bytes().as_slice()).unwrap();
    sign_file.write_all(s.as_slice()).unwrap();
    sign_file.flush().unwrap();
}

pub fn verify_file<P: AsRef<Path>>(path: P, sign_file_path: P, y: &BigUint, g: &BigUint, p: &BigUint) {
    let binding = fs::read(path).unwrap();
    let msg_bytes = binding.as_slice();
    let bytes = fs::read(&sign_file_path).unwrap();
    let sign_file = fs::File::open(sign_file_path).unwrap();
    let mut reader = io::BufReader::new(sign_file);
    
    let mut r_len = [0u8; core::mem::size_of::<usize>()];
    reader.read_exact(&mut r_len).unwrap();
    let r_len = usize::from_le_bytes(r_len);
    let mut r = vec![0u8; r_len];
    reader.read_exact(&mut r).unwrap();
    
    let mut s_len = [0u8; core::mem::size_of::<usize>()];
    reader.read_exact(&mut s_len).unwrap();
    let s_len = usize::from_le_bytes(s_len);
    let mut s = vec![0u8; s_len];
    reader.read_exact(&mut s).unwrap();
    
    let r = num_primes::BigUint::from_bytes_le(r.as_slice());
    let s = num_primes::BigUint::from_bytes_le(s.as_slice());
    verify(msg_bytes, y, &r, &s, g, p);
}