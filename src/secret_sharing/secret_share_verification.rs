use std::error::Error;

use num::One;
use num_bigint::BigInt;

use super::secret_splitting::LagrangePolynomial;

#[derive(Debug)]
pub struct Commitments {
    coefficients: Vec<BigInt>,
}
impl Commitments {
    pub fn generate_commitments(
        generator: &BigInt,
        polynomial: &LagrangePolynomial,
        PRIME_MODULUS: i32,
    ) -> Result<Self, Box<dyn Error>> {
        if polynomial.poly.is_empty() {
            return Err("Polynomial has no coefficients".into());
        }

        let mut coefficients = vec![];

        for exp in &polynomial.poly {
            let commitment = generator.modpow(exp, &BigInt::from(PRIME_MODULUS));
            coefficients.push(commitment);
        }

        Ok(Commitments { coefficients })
    }

    pub fn verify_share(
        x: &BigInt,
        y: &BigInt,
        generator: &BigInt,
        commitments: &Commitments,
        PRIME_MODULUS: i32,
    ) -> bool {
        let mut computed_commitment = BigInt::one();
        let mut power_of_x = BigInt::one();

        for coeff_commitment in &commitments.coefficients {
            computed_commitment = (computed_commitment
                * coeff_commitment.modpow(&power_of_x, &BigInt::from(PRIME_MODULUS)))
                % PRIME_MODULUS;
            power_of_x = (power_of_x * x) % PRIME_MODULUS;
        }

        generator.modpow(y, &BigInt::from(PRIME_MODULUS)) == computed_commitment
    }
}
