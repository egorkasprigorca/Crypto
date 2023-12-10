use std::fs;

use num_primes::Generator;
use rand::Rng;

// 00-red, 01-blue, 10-yellow
enum Color {
    RED,
    BLUE,
    YELLOW
}

fn main() {
    let edges: Vec<(usize, usize)> = vec![(1, 2), (2, 3), (2, 4), (2, 5), (4, 5), (5, 6), (3, 4)];
    let vertices = 6;
    let colors = vec![Color::YELLOW, Color::BLUE, Color::YELLOW, Color::RED, Color::YELLOW, Color::BLUE];

    let file_contents = fs::read_to_string("src/data/graph").expect("Unable to read file");

    for line in file_contents.lines() {
        println!("{}", line);
    }

    let mut rng = rand::thread_rng();
    let mut rs = Vec::with_capacity(vertices);
    for i in 1..vertices {
        let r = rng.gen::<u64>();
        let left_bits = r & !0b11;
        let right_bits = match colors[i] {
            Color::RED => 0b00,
            Color::BLUE => 0b01,
            Color::YELLOW => 0b10,
        };
        let r = left_bits | right_bits;
        let r = num_primes::BigUint::from_bytes_le(&r.to_le_bytes());
        rs.push(r);
    }

    let mut rsa_params = Vec::with_capacity(vertices);
    for i in 1..vertices {
        let (c, d, n, p, q) = enc::rsa_ds::init(512);
        rsa_params.push((c, d, n, p, q));
    }
    let mut zs = Vec::with_capacity(vertices);
    for i in 1..vertices {
        let z = rs[i-1].modpow(&rsa_params[i-1].1, &rsa_params[i-1].2);
        zs.push(z);
    }

    for i in 1..edges.len() {
        let edge = edges[i-1];
        let c1 = rsa_params[edge.0-1].0.clone();
        let c2 = rsa_params[edge.1-1].0.clone();
        let z1 = zs[edge.0-1].clone().modpow(&c1, &rsa_params[edge.0-1].2);
        let z2 = zs[edge.1-1].clone().modpow(&c2, &rsa_params[edge.1-1].2);
        let last_2_bits_z1 = z1.to_u32_digits().last().unwrap() & 0b11;
        let last_2_bits_z2 = z2.to_u32_digits().last().unwrap() & 0b11;
        if last_2_bits_z1 == last_2_bits_z2 {
            println!("Совпали!!!!!!!!!!");
            break;
        }
    }
    
}
