use std::{path::Path, fs, io::{Write, self, Read}, borrow::Cow};

use num::{One, Zero};
use num_primes::{Generator, BigUint};
use sha2::{Sha224, Digest};

use crate::util::num_inv_by_mod;

pub fn init() -> (BigUint, BigUint, BigUint, BigUint, BigUint) {
    let (p, q) = loop {
        // diff between ru and us here
        let q = Generator::new_prime(160);
        let p: BigUint = BigUint::one() % BigUint::from(2u8)*&q;
        if num_primes::Verification::is_prime(&p) {
           break (p, q) 
        }
    };
    
    let a = BigUint::from(2u8).modpow(&((&p - BigUint::one()) / &q), &p);
    // Private key
    let x = loop {
        // diff between ru and us here
        let n = Generator::new_uint(160);
        if n < q {
            break n;
        }
    };
    // Public key
    let y = a.modpow(&x, &p);
    (p, q, a, x, y)
}

fn sign(message: &[u8], a: &BigUint, p: &BigUint, q: &BigUint, x: &BigUint) -> (BigUint, BigUint) {
    let mut hasher = Sha224::new();
    hasher.update(message);
    let h = hasher.finalize();
    let h = BigUint::from_bytes_le(&h);

    let (r, s) = loop {
        let k = loop {
            let k = Generator::new_uint(128);
            if k < *q {
                break k
            }
        };
        let r = a.modpow(&k, &p) % q;
        if r == BigUint::zero() {
            continue
        }
        // diff between ru and us here
        // let k_inv_mod_q = num_bigint::algorithms::mod_inverse(
        //     Cow::Owned(num_bigint::BigUint::from_bytes_le(&*k.to_bytes_le())),
        //     Cow::Owned(num_bigint::BigUint::from_bytes_le(&*q.to_bytes_le()))
        // ).unwrap();
        // let k_inv_mod_q = num_primes::BigUint::from_bytes_le(&*k_inv_mod_q.to_bytes_le().1);
        let k_inv_mod_q = num_inv_by_mod(&k, q);

        let s = k_inv_mod_q * ((&h + x*&r) % q);
        if s == BigUint::zero() {
            continue;
        }
        break (r, s)
    };
    (r, s)
}

fn verify(message: &[u8], r: &BigUint, s: &BigUint, p: &BigUint, q: &BigUint, a: &BigUint, y: &BigUint) {
    if *r>=*q || *s>=*q {
        return;
    }

    let mut hasher = Sha224::new();
    hasher.update(message);
    let h = hasher.finalize();
    let h = BigUint::from_bytes_le(&h);

    // let s_inv_mod_q = num_bigint::algorithms::mod_inverse(
    //     Cow::Owned(num_bigint::BigUint::from_bytes_le(&*s.to_bytes_le())),
    //     Cow::Owned(num_bigint::BigUint::from_bytes_le(&*q.to_bytes_le()))
    // ).unwrap();
    // let s_inv_mod_q = num_primes::BigUint::from_bytes_le(&*s_inv_mod_q.to_bytes_le().1);
    let s_inv_mod_q = num_inv_by_mod(s, q);

    // diff between ru and us here
    let u1 = &s_inv_mod_q * (h % q);
    let u2 = &s_inv_mod_q * (r % q);
    let u = ((a.modpow(&u1, p)*(y.modpow(&u2, p))) % p) % q;
    assert_eq!(u, *r);
}

pub fn sign_file<P: AsRef<Path>>(path: P, sign_file_path: P, a: &BigUint, p: &BigUint, q: &BigUint, x: &BigUint) {
    let binding = fs::read(path).unwrap();
    let msg_bytes = binding.as_slice();
    let (r, s) = sign(msg_bytes, a, p, q, x);
    let r = r.to_bytes_le();
    let s = s.to_bytes_le();
    let mut sign_file = fs::File::create(sign_file_path).unwrap();
    sign_file.write_all(r.len().to_le_bytes().as_slice()).unwrap();
    sign_file.write_all(r.as_slice()).unwrap();
    sign_file.write_all(s.len().to_le_bytes().as_slice()).unwrap();
    sign_file.write_all(s.as_slice()).unwrap();
    sign_file.flush().unwrap();
}

pub fn verify_file<P: AsRef<Path>>(path: P, sign_file_path: P, p: &BigUint, q: &BigUint, a: &BigUint, y: &BigUint) {
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
    verify(msg_bytes, &r, &s, p, q, a, y);
}