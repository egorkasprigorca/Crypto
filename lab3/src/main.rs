use std::time::Instant;

extern crate num_bigint_dig as num_bigint;

mod util;
mod rsa_ds;
mod elgamal_ds;
mod dsa_ru;
mod dsa_us;

fn do_rsa_ds(dir: &String) {
    let start = Instant::now();

    let (c, d, n) = rsa_ds::init(1024);
    rsa_ds::sign_file(
        dir.to_owned() + "logo.jpg", 
        dir.to_owned() + "sign_rsa", 
        &c, &n
    );
    rsa_ds::verify_file(
        dir.to_owned() + "logo.jpg", 
        dir.to_owned() + "sign_rsa", 
        &d, &n
    );

    let duration = start.elapsed();
    println!("elapsed: {:#?}", duration);
}

fn do_elgamal_ds(dir: &String) {
    let start = Instant::now();
    
    let (p, g, x, y) = elgamal_ds::init(1024);
    elgamal_ds::sign_file(
        dir.to_owned() + "logo.jpg", 
        dir.to_owned() + "sign_elgamal", 
        &p, &g, &x
    );
    elgamal_ds::verify_file(
        dir.to_owned() + "logo.jpg", 
        dir.to_owned() + "sign_elgamal", 
        &y, &g, &p
    );

    let duration = start.elapsed();
    println!("elapsed: {:#?}", duration);
}

fn do_dsa_ru(dir: &String) {
    let start = Instant::now();

    let (p, q, a, x, y) = dsa_ru::init();
    dsa_ru::sign_file(
        dir.clone() + "logo.jpg", 
        dir.clone() + "dsa_ru_sign", 
        &a, &p, &q, &x
    );
    dsa_ru::verify_file(
        dir.clone() + "logo.jpg", 
        dir.clone() + "dsa_ru_sign", 
        &p, &q, &a, &y
    );

    let duration = start.elapsed();
    println!("elapsed: {:#?}", duration);
}

fn do_dsa_us(dir: &String) {
    let start = Instant::now();

    let (p, q, a, x, y) = dsa_us::init();
    dsa_us::sign_file(
        dir.clone() + "logo.jpg", 
        dir.clone() + "dsa_us_sign", 
        &a, &p, &q, &x
    );
    dsa_us::verify_file(
        dir.clone() + "logo.jpg", 
        dir.clone() + "dsa_us_sign", 
        &p, &q, &a, &y
    );

    let duration = start.elapsed();
    println!("elapsed: {:#?}", duration);
}

fn main() {
    let dir = String::from("C:/Users/egor/Desktop/Projects/Crypto/data/");

    do_rsa_ds(&dir);
    do_elgamal_ds(&dir);
    do_dsa_ru(&dir);
    do_dsa_us(&dir);
}
