use std::error::Error;

use num_bigint::BigInt;
use num_traits::{One, Zero};
use rand::Rng;

#[derive(Debug)]
pub struct LagrangePolynomial {
    pub poly: Vec<BigInt>,
}

impl LagrangePolynomial {
    pub fn generate_polynomial(
        secret: BigInt,
        threshold: BigInt,
        PRIME_MODULUS: i32,
    ) -> Result<LagrangePolynomial, Box<dyn Error>> {
        let mut polynomial = LagrangePolynomial { poly: vec![] };

        polynomial.poly.push(secret);

        let mut rnd = rand::thread_rng();
        let mut i = BigInt::one();

        while i < threshold {
            let mut random_coefficient = BigInt::from(0);

            loop {
                if random_coefficient != BigInt::from(0) {
                    break;
                }

                let random_number: i32 = rnd.gen_range(1..PRIME_MODULUS);
                random_coefficient = BigInt::from(random_number);
            }

            polynomial.poly.push(BigInt::from(random_coefficient));

            i += BigInt::one();
        }

        Ok(polynomial)
    }
}

#[derive(Debug)]
pub struct Share {
    pub x: BigInt,
    pub y: BigInt,
}

#[derive(Debug)]
pub struct Shares {
    pub shares: Vec<Share>,
}

impl Shares {
    pub fn generate_n_shares(
        polynomial: &LagrangePolynomial,
        no_of_shares: BigInt,
    ) -> Result<Shares, Box<dyn Error>> {
        let mut shares = Shares { shares: vec![] };

        let mut i: BigInt = BigInt::one();

        while i <= no_of_shares {
            let x: BigInt = i.clone();
            let y: BigInt = Shares::generate_y_point(&polynomial, &x);

            shares.shares.push(Share { x, y });
            i += BigInt::one();
        }

        Ok(shares)
    }

    fn generate_y_point(polynomial: &LagrangePolynomial, x: &BigInt) -> BigInt {
        let mut y = BigInt::zero();
        let mut temp = BigInt::one();

        // Iterating through the coefficients to compute y

        for coeff in &polynomial.poly {
            y = y + coeff * &temp;
            temp = temp * x;
        }

        y
    }
}
