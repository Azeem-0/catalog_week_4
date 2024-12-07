use std::error::Error;

use num_bigint::BigInt;
use num_traits::{One, Zero};
use rand::Rng;

const PRIME_MODULUS: i32 = 997;

// should make this one module.

#[derive(Debug)]
pub struct Polynomial {
    pub poly: Vec<BigInt>,
}

pub fn generate_polynomial(
    secret: BigInt,
    threshold: BigInt,
) -> Result<Polynomial, Box<dyn Error>> {
    let mut polynomial = Polynomial { poly: vec![] };

    polynomial.poly.push(secret);

    let mut rnd = rand::thread_rng();
    let mut i = BigInt::one();

    while i < threshold {
        let mut p = BigInt::from(0);

        loop {
            if p != BigInt::from(0) {
                break; // Ensure the coefficient is not zero
            }

            let random_number: i32 = rnd.gen_range(1..PRIME_MODULUS); // Generate random number in the range [1, PRIME_MODULUS)
            p = BigInt::from(random_number); // Convert to BigInt
        }

        polynomial.poly.push(BigInt::from(p));

        i += BigInt::one(); // Increment by 1 (BigInt equivalent)
    }

    Ok(polynomial)

    // generate the random coefficients for the polynomial
}

// should make this one module.
#[derive(Debug)]
pub struct Share {
    pub x: BigInt,
    pub y: BigInt,
}

#[derive(Debug)]
pub struct Shares {
    pub shares: Vec<Share>,
}

fn generate_y_point(polynomial: &Polynomial, x: &BigInt) -> BigInt {
    let mut y = BigInt::zero();
    let mut temp = BigInt::one();

    // Iterating through the coefficients to compute y

    for coeff in &polynomial.poly {
        y = y + coeff * &temp;
        temp = temp * x;
    }

    y
}

pub fn generate_n_shares(
    polynomial: &Polynomial,
    no_of_shares: BigInt,
) -> Result<Shares, Box<dyn Error>> {
    let mut shares = Shares { shares: vec![] };

    let mut i: BigInt = BigInt::one();

    while i <= no_of_shares {
        let x: BigInt = i.clone();
        let y: BigInt = generate_y_point(&polynomial, &x);

        shares.shares.push(Share { x, y });
        i += BigInt::one();
    }

    Ok(shares)
}

// should make this one module.

// Fraction structure to handle the fractions
#[derive(Debug, Clone)]
struct Fraction {
    num: BigInt,
    den: BigInt,
}

impl Fraction {
    fn new(num: BigInt, den: BigInt) -> Self {
        if den.is_zero() {
            panic!("Denominator cannot be zero");
        }
        Fraction { num, den }
    }

    // Multiplying two fractions
    fn multiply(&self, other: &Fraction) -> Fraction {
        let num = &self.num * &other.num;
        let den = &self.den * &other.den;
        Fraction::new(num, den)
    }
}

pub fn reconstruct_secret(
    shares: &Vec<Share>,
    k_shares: usize,
    threshold: BigInt,
) -> Result<BigInt, Box<dyn std::error::Error>> {
    let mut secret: BigInt = BigInt::zero();

    // Ensure there are enough shares (>= threshold)
    if BigInt::from(k_shares) < threshold {
        return Err("Not enough shares to reconstruct the secret".into());
    }

    for i in 0..k_shares {
        let xi = &shares[i].x;
        let yi = &shares[i].y;

        let mut lagrange = Fraction::new(One::one(), One::one()); // Initialize Lagrange term as 1/1

        // Compute Lagrange basis polynomial for the i-th share
        for j in 0..k_shares {
            if i != j {
                let xj = &shares[j].x;

                // Compute the fraction (x - xj) / (xi - xj)
                let numerator: BigInt = -1 * xj;
                let denominator: BigInt = xi - xj;

                let temp = Fraction::new(numerator.clone(), denominator.clone());

                lagrange = lagrange.multiply(&temp);
            }
        }

        // Multiply the Lagrange polynomial term by the corresponding yi

        let result = (&lagrange.num / &lagrange.den) * yi;

        secret += result;
    }

    Ok(secret)
}
