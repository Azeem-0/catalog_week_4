use num::BigInt;
use num_traits::ToPrimitive;

use crate::secret_sharing::{
    secret_reconstruction::reconstruct_secret,
    secret_share_verification::Commitments,
    secret_splitting::{LagrangePolynomial, Shares},
};

const PRIME_MODULUS: i32 = 997;

pub fn verifiable_secret_sharing(
    secret: BigInt,
    no_of_shares: BigInt,
    threshold: BigInt,
    generator: BigInt,
) {
    let polynomial: LagrangePolynomial =
        LagrangePolynomial::generate_polynomial(secret, threshold.clone(), PRIME_MODULUS).unwrap();

    println!("{:?}", polynomial);

    let shares = Shares::generate_n_shares(&polynomial, no_of_shares).unwrap();

    let commitments: Commitments =
        Commitments::generate_commitments(&generator, &polynomial, PRIME_MODULUS).unwrap();

    for (index, share) in shares.shares.iter().enumerate() {
        let validation =
            Commitments::verify_share(&share.x, &share.y, &generator, &commitments, PRIME_MODULUS);

        println!(
            "Share {} is {}",
            index + 1,
            if validation { "valid" } else { "invalid" }
        );
    }

    let reconstructed_secret = reconstruct_secret(&shares.shares, 3, threshold).unwrap();

    println!("Reconstructed Secret : {}", reconstructed_secret);
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigInt;
    use num_traits::{One, Zero};

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
