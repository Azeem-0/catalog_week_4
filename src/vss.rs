use std::error::Error;

use num::BigInt;
use num_traits::ToPrimitive;

use crate::secret_sharing::{
    secret_reconstruction::reconstruct_secret,
    secret_share_verification::Commitments,
    secret_splitting::{LagrangePolynomial, Shares},
};

pub fn verifiable_secret_sharing(
    secret: BigInt,
    no_of_shares: BigInt,
    threshold: BigInt,
    prime_modulus: i32,
) -> Result<(), Box<dyn Error>> {
    let generator = BigInt::from(5);

    let polynomial: LagrangePolynomial =
        match LagrangePolynomial::generate_polynomial(secret, threshold.clone(), prime_modulus) {
            Ok(pol) => pol,
            Err(err) => {
                println!("{:?}", err);
                return Err(err);
            }
        };

    println!("{:?}\n", polynomial);

    let shares = match Shares::generate_n_shares(&polynomial, no_of_shares) {
        Ok(sh) => sh,
        Err(err) => return Err(err),
    };

    let k_shares = 3;

    let commitments: Commitments =
        match Commitments::generate_commitments(&generator, &polynomial, prime_modulus) {
            Ok(com) => com,
            Err(err) => {
                println!("{}", err);
                return Err(err);
            }
        };

    for (index, share) in shares.shares.iter().enumerate() {
        let validation =
            Commitments::verify_share(&share.x, &share.y, &generator, &commitments, prime_modulus);

        println!(
            "Share {} is {}",
            index + 1,
            if validation { "valid" } else { "invalid" }
        );
    }

    let reconstructed_secret = match reconstruct_secret(&shares.shares, k_shares, threshold) {
        Ok(sec) => sec,
        Err(err) => {
            println!("{}", err);
            return Err(err);
        }
    };

    println!("Reconstructed Secret : {}", reconstructed_secret);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigInt;

    const PRIME_MODULUS: i32 = 997;

    #[test]
    fn test_verify_share() {
        let secret = BigInt::from(123);
        let threshold = BigInt::from(3);
        let polynomial =
            LagrangePolynomial::generate_polynomial(secret, threshold, PRIME_MODULUS).unwrap();

        let no_of_shares = BigInt::from(5);

        let shares = Shares::generate_n_shares(&polynomial, no_of_shares.clone()).unwrap();

        let generator = BigInt::from(2);

        let commitments =
            Commitments::generate_commitments(&generator, &polynomial, PRIME_MODULUS).unwrap();

        for share in &shares.shares {
            let is_valid = Commitments::verify_share(
                &share.x,
                &share.y,
                &generator,
                &commitments,
                PRIME_MODULUS,
            );
            assert!(is_valid);
        }
    }

    #[test]
    fn test_reconstruct_secret() {
        let secret = BigInt::from(123);
        let threshold = BigInt::from(3);
        let polynomial = LagrangePolynomial::generate_polynomial(
            secret.clone(),
            threshold.clone(),
            PRIME_MODULUS,
        )
        .unwrap();

        let no_of_shares = BigInt::from(5);
        let shares = Shares::generate_n_shares(&polynomial, no_of_shares.clone()).unwrap();

        // Reconstruct using first 'threshold' shares
        let reconstructed_secret =
            reconstruct_secret(&shares.shares, threshold.to_usize().unwrap(), threshold).unwrap();

        assert_eq!(reconstructed_secret, secret);
    }

    #[test]
    fn test_invalid_share_verification() {
        let secret = BigInt::from(123);
        let threshold = BigInt::from(3);
        let polynomial =
            LagrangePolynomial::generate_polynomial(secret, threshold, PRIME_MODULUS).unwrap();

        let no_of_shares = BigInt::from(5);
        let mut shares = Shares::generate_n_shares(&polynomial, no_of_shares.clone()).unwrap();

        let generator = BigInt::from(2);
        let commitments =
            Commitments::generate_commitments(&generator, &polynomial, PRIME_MODULUS).unwrap();

        // Corrupt the first share
        shares.shares[0].y += BigInt::from(1);

        // Verify shares
        let is_valid = Commitments::verify_share(
            &shares.shares[0].x,
            &shares.shares[0].y,
            &generator,
            &commitments,
            PRIME_MODULUS,
        );

        assert!(!is_valid);
    }
}
