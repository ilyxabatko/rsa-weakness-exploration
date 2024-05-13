use num::{FromPrimitive, Zero};
use num_bigint::BigUint;
use num_primes::{BigUint as BigUintPrimes, Generator, Verification};
use num_traits::One;

fn main() {
    let n_biguint_primes = generate_modulus();
    println!("Generated n: {}", n_biguint_primes);

    let n: BigUint = BigUint::from_bytes_be(&n_biguint_primes.to_bytes_be());
    let a = &n.sqrt() + &BigUint::one();
    let b: BigUint;
    let e = BigUint::from_u32(65537).unwrap();

    let mut a_pow = a.pow(2);

    loop {
        let b2 = &a_pow - &n;

        let possible_sqrt = &b2.sqrt();

        if possible_sqrt.pow(2) == b2 {
            b = possible_sqrt.clone();
            break;
        }

        a_pow += BigUint::one()
    }

    let p = &a + &b;
    let q = &a - &b;

    let p_is_prime = Verification::is_prime(&BigUintPrimes::from_bytes_be(&p.to_bytes_be()));
    let q_is_prime = Verification::is_prime(&BigUintPrimes::from_bytes_be(&q.to_bytes_be()));

    println!("P is prime: {}", p_is_prime);
    println!("Q is prime: {}", q_is_prime);

    println!("P: {}", &p);
    println!("Q: {}", &q);

    let n_result = &p * &q;
    println!("Result n: {}", n_result);

    if &n_result == &n {
        println!("Found n: {}", &n);
    } else {
        panic!("Generated 'n' isn't equal to 'n_result'");
    }

    let phi_n = (&p.clone() - &BigUint::one()) * (&q - &BigUint::one());
    let d = mod_inverse(e, phi_n);
    println!("Private key: {}", &d);
}

fn generate_modulus() -> BigUintPrimes {
    let similar_first_half = Generator::new_prime(500);
    let mut p = Generator::new_prime(200) + &similar_first_half;
    let mut q = Generator::new_prime(200) + &similar_first_half;

    while !Verification::is_prime(&p) {
        p += BigUintPrimes::from_u8(1).unwrap();
    }

    while !Verification::is_prime(&q) {
        q += BigUintPrimes::from_u8(1).unwrap();
    }

    let p_is_prime = Verification::is_prime(&p);
    let q_is_prime = Verification::is_prime(&q);

    println!("P is prime: {}", p_is_prime);
    println!("Q is prime: {}", q_is_prime);

    let n = p * q;
    return n;
}

fn modular_exponent(mut n: BigUint, mut x: BigUint, p: BigUint) -> BigUint {
    let mut ans = BigUint::one();
    if x <= BigUint::zero() {
        return BigUint::one();
    }

    loop {
        if x.clone() == BigUint::one() {
            return (ans * n.clone()) % p.clone();
        }

        if x.clone() & BigUint::one() == BigUint::zero() {
            n = (n.clone() * n.clone()) % p.clone();
            x >>= 1;
            continue;
        } else {
            ans = (ans * n.clone()) % p.clone();
            x -= BigUint::one();
        }
    }
}
// Find greatest common denominator
fn gcd(mut a: BigUint, mut b: BigUint) -> BigUint {
    if a == b.clone() {
        return a;
    }
    if b > a {
        let temp = a;
        a = b.clone();
        b = temp;
    }
    while b > BigUint::zero() {
        let temp = a;
        a = b.clone();
        b = temp % b.clone();
    }
    return a;
}

fn mod_inverse(n: BigUint, p: BigUint) -> BigUint {
    if p <= BigUint::one() || gcd(n.clone(), p.clone()) > BigUint::one() {
        return BigUint::zero();
    }

    return modular_exponent(n, p.clone() - BigUint::from_u8(2).unwrap(), p);
}
