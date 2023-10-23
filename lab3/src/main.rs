use std::time::Instant;

use num::One;
use num_primes::BigUint;
use num_primes::Generator;
use sha3::{Digest, Sha3_512};
use util::gcd;
use util::num_inv_by_mod;

extern crate num_bigint_dig as num_bigint;

mod dsa_ru;
mod dsa_us;
mod elgamal_ds;
mod rsa_ds;
mod util;

// fn do_rsa_ds(dir: &String) {
//     let start = Instant::now();

//     let (c, d, n) = rsa_ds::init(1024);
//     rsa_ds::sign_file(
//         dir.to_owned() + "logo.jpg",
//         dir.to_owned() + "sign_rsa",
//         &c, &n
//     );
//     rsa_ds::verify_file(
//         dir.to_owned() + "logo.jpg",
//         dir.to_owned() + "sign_rsa",
//         &d, &n
//     );

//     let duration = start.elapsed();
//     println!("elapsed: {:#?}", duration);
// }

// fn do_elgamal_ds(dir: &String) {
//     let start = Instant::now();

//     let (p, g, x, y) = elgamal_ds::init(1024);
//     elgamal_ds::sign_file(
//         dir.to_owned() + "logo.jpg",
//         dir.to_owned() + "sign_elgamal",
//         &p, &g, &x
//     );
//     elgamal_ds::verify_file(
//         dir.to_owned() + "logo.jpg",
//         dir.to_owned() + "sign_elgamal",
//         &y, &g, &p
//     );

//     let duration = start.elapsed();
//     println!("elapsed: {:#?}", duration);
// }

// fn do_dsa_ru(dir: &String) {
//     let start = Instant::now();

//     let (p, q, a, x, y) = dsa_ru::init();
//     dsa_ru::sign_file(
//         dir.to_owned() + "logo.jpg",
//         dir.to_owned() + "dsa_ru_sign",
//         &a, &p, &q, &x
//     );
//     dsa_ru::verify_file(
//         dir.to_owned() + "logo.jpg",
//         dir.to_owned() + "dsa_ru_sign",
//         &p, &q, &a, &y
//     );

//     let duration = start.elapsed();
//     println!("elapsed: {:#?}", duration);
// }

// fn do_dsa_us(dir: &String) {
//     let start = Instant::now();

//     let (p, q, a, x, y) = dsa_us::init();
//     dsa_us::sign_file(
//         dir.to_owned() + "logo.jpg",
//         dir.to_owned() + "dsa_us_sign",
//         &a, &p, &q, &x
//     );
//     dsa_us::verify_file(
//         dir.to_owned() + "logo.jpg",
//         dir.to_owned() + "dsa_us_sign",
//         &p, &q, &a, &y
//     );

//     let duration = start.elapsed();
//     println!("elapsed: {:#?}", duration);
// }

// fn main() {
//     let dir = String::from("C:/Users/Егор/Documents/Github/Crypto/data/");
//     do_rsa_ds(&dir);
//     do_elgamal_ds(&dir);
//     do_dsa_ru(&dir);
//     do_dsa_us(&dir);
// }

//client
fn vote(answer: bool, d: &BigUint) -> (BigUint, BigUint) {
    let rnd = Generator::new_uint(512);
    let v = BigUint::from_bytes_le(answer.to_string().as_bytes());
    let n = rnd | v;
    let r = loop {
        let r = Generator::new_uint(512);
        if gcd(&r, &n) == BigUint::one() {
            break r;
        }
    };
    let h = loop {
        let mut hasher = Sha3_512::new();
        hasher.update(n.to_bytes_le());
        let h = hasher.finalize();
        let h = BigUint::from_bytes_le(&h);
        if h < n {
            break h;
        }
    };
    let left = h % &n;
    let right = r.modpow(d, &n);
    let h = (left * right) % &n;
    (h, r)
}

//server

fn main() {
    // lab5

    // server knows c, user d, n
    let (c, d, n) = rsa_ds::init(1024);
    let (h, r) = vote(true, &d);
    //send h to server
    //server
    let s = h.modpow(&c, &n);
    //send s to alice
    let left = s % &n;
    let right = num_inv_by_mod(&r, &n);
    let s = left * right;
    // send n, s to server then server verifies it
    let mut hasher = Sha3_512::new();
    hasher.update(n.to_bytes_le());
    let h = hasher.finalize();
    let left = BigUint::from_bytes_le(&h);
    let right = s.modpow(&d, &n);
    assert_eq!(left, right);
}
