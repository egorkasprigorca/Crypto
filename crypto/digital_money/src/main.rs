extern crate enc;

use enc::{rsa_ds, util::{gcd, num_inv_by_mod}};
use num::One;
use num_primes::{BigUint, Generator};

struct Banknote {
    cost: u32,
    number: u32,
    c: BigUint,
    d: BigUint
}

fn get_banknote(banknote: &Banknote, n: &BigUint) -> (BigUint, BigUint) {
    let r = loop {
        let r = Generator::new_uint(1024);
        if gcd(&r, n) == BigUint::one() {
            break r;
        }
    };
    let n2 = (banknote.number % n) * r.modpow(&banknote.d, n);
    (n2, r)
}

fn main() {
    let (vec_c, vec_d, n, p, q) = rsa_ds::init_many_c_d(1024, 4);
    let _1000rub = Banknote {
        cost: 1000,
        number: 99,
        c: vec_c[0].clone(),
        d: vec_d[0].clone()
    };
    let _500rub = Banknote {
        cost: 500,
        number: 55,
        c: vec_c[1].clone(),
        d: vec_d[1].clone()
    };
    let _100rub = Banknote {
        cost: 100,
        number: 11,
        c: vec_c[2].clone(),
        d: vec_d[2].clone()
    };
    let (n2, r) = get_banknote(&_1000rub, &n);
    // bank: user - cost banknote
    let s2 = n2.modpow(&_1000rub.c, &n);
    // user:
    let r_inv_mod_n = num_inv_by_mod(&r, &n);
    let s = (s2 * r_inv_mod_n) % &n;
    // (n, s) == banknote
    // user sends it to shop
    // shop sends it to server
    let check = n2.modpow(&_1000rub.d, &n);
    assert_eq!(check, s)
}
