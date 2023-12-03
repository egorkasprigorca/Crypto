extern crate enc;

use std::collections::HashMap;

use enc::util::{gcd, num_inv_by_mod, sha3_256};
use num::{FromPrimitive, Integer, One};
use num_primes::{BigUint, Generator};

enum Vote {
    YES,
    NO,
    REJECT,
}

struct Params {
    C: BigUint,
    D: BigUint,
    N: BigUint,
    P: BigUint,
    Q: BigUint,
}

fn reg_vote(
    params: &Params,
    votes: &mut HashMap<u32, bool>,
    voter_id: u32,
    h_: BigUint,
) -> BigUint {
    let is_voted = votes.get(&voter_id);
    let _ = match is_voted {
        Some(_) => return BigUint::one(),
        None => 10,
    };
    votes.insert(voter_id, true);
    h_.modpow(&params.C, &params.N)
}

fn check(params: &Params, voter_id: u32, n: BigUint, s: BigUint) {
    let hash = sha3_256(n);
    let right = s.modpow(&params.D, &params.N);
    assert_eq!(hash, right);
    println!("Newsletter is correct for voter: {voter_id}");
}

fn vote(vote: Vote, params: &Params, votes: &mut HashMap<u32, bool>, voter_id: u32) {
    let v = match vote {
        Vote::YES => 1,
        Vote::NO => 2,
        Vote::REJECT => 3,
    };
    let rnd = Generator::new_uint(512);
    let shift = rnd << 512;
    let left = num::BigUint::from_bytes_le(&shift.to_bytes_le());
    let right = num::BigUint::from_i32(v).unwrap();
    let n = left | right;
    let n = num_primes::BigUint::from_bytes_le(&n.to_bytes_le());

    let r = loop {
        let r = Generator::new_uint(512);
        if gcd(&r, &params.N) == BigUint::one() {
            break r;
        }
    };
    let h = loop {
        let h = sha3_256(n.clone());
        if h < params.N {
            break h;
        }
    };  
    let h_ = ((h % &params.N) * r.modpow(&params.D, &params.N)) % &params.N;
    let s_ = reg_vote(params, votes, voter_id, h_.clone());
    if s_ == BigUint::one() {
        println!("Already voted voter: {voter_id}\n\n");
        return;
    }
    let s: BigUint = ((&s_ % &params.N) * num_inv_by_mod(&r, &params.N)) % &params.N;
    check(params, voter_id, n.clone(), s.clone());
    println!("h_: {h_}\ns_: {s_}\ns: {s}\nn: {n}\n");
    println!("\n\n");
}

fn main() {
    let mut votes: HashMap<u32, bool> = HashMap::new();
    let (C, D, N, P, Q) = enc::rsa_ds::init(1024);
    let params = Params { C, D, N, P, Q };
    // P, Q, C = SECRET 
    // N, D = PUBLIC
    println!("C: {}\n\nD: {}\n\nN: {}\n\nP: {}\n\nQ: {}\n\n", params.C, params.D, params.N, params.P, params.Q);
    vote(Vote::REJECT, &params, &mut votes, 12);
    vote(Vote::YES, &params, &mut votes, 12);
    vote(Vote::NO, &params, &mut votes, 2342);
    vote(Vote::REJECT, &params, &mut votes, 152);
    vote(Vote::REJECT, &params, &mut votes, 152);
}