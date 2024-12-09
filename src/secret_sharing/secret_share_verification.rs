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
        prime_modulus: i32,
    ) -> Result<Self, Box<dyn Error>> {
        if polynomial.poly.is_empty() {
            return Err("Polynomial has no coefficients".into());
        }

        let mut commitment_coefficients = vec![];

        for exp in &polynomial.poly {
            let commitment = generator.modpow(exp, &BigInt::from(prime_modulus));
            commitment_coefficients.push(commitment);
        }

        Ok(Commitments {
            coefficients: commitment_coefficients,
        })
    }

    pub fn verify_share(
        x: &BigInt,
        y: &BigInt,
        generator: &BigInt,
        commitments: &Commitments,
        prime_modulus: i32,
    ) -> bool {
        let mut computed_commitment = BigInt::one();
        let mut power_of_x = BigInt::one();

        for coeff_commitment in &commitments.coefficients {
            computed_commitment = (computed_commitment
                * coeff_commitment.modpow(&power_of_x, &BigInt::from(prime_modulus)))
                % prime_modulus;
            power_of_x = (power_of_x * x) % prime_modulus;
        }

        generator.modpow(y, &BigInt::from(prime_modulus)) == computed_commitment
    }
}
