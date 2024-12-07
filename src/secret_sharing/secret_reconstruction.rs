use num_bigint::BigInt;
use num_traits::{One, Zero};

use super::secret_splitting::Share;

#[derive(Debug, Clone)]
pub struct Fraction {
    pub num: BigInt,
    pub den: BigInt,
}

impl Fraction {
    pub fn new(num: BigInt, den: BigInt) -> Self {
        if den.is_zero() {
            panic!("Denominator cannot be zero");
        }
        Fraction { num, den }
    }

    // Multiplying two fractions
    pub fn multiply(&self, other: &Fraction) -> Fraction {
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
                let numerator: BigInt = BigInt::from(-1) * xj;
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
