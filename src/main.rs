use num_bigint::BigUint;
use num_traits::{One, Zero};
use num_integer::Integer; // Import Integer trait for gcd method
use std::str::FromStr;

fn pollards_rho(n: &BigUint) -> BigUint {
    let mut x = BigUint::from(2u32);
    let mut y = BigUint::from(2u32);
    let mut d = BigUint::one();
    let one = BigUint::one();
    let f = |z: &BigUint| (z * z + &one) % n;

    while d == BigUint::one() {
        x = f(&x);
        y = f(&f(&y));
        let abs_diff = if &x > &y { &x - &y } else { &y - &x };
        d = abs_diff.gcd(n);

        // Break loop if we get a non-trivial divisor
        if d != BigUint::one() && d != *n {
            break;
        }
    }

    d
}

fn main() {
    let number_str = "59416033658004120313555424513491018615317021176268174958724971404913";
    let n = BigUint::from_str(number_str).unwrap();

    if n.is_zero() {
        println!("Number is zero, no factors.");
        return;
    }
    if n.is_one() {
        println!("Number is one, no factors.");
        return;
    }

    // Basic factorization using Pollard's Rho
    let factor = pollards_rho(&n);

    println!("Factor is: {}", factor);
}


