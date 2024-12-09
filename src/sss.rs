use std::error::Error;

use num_bigint::BigInt;
#[allow(unused_imports)]
use num_traits::ToPrimitive;

use crate::secret_sharing::{
    secret_reconstruction::reconstruct_secret,
    secret_splitting::{LagrangePolynomial, Shares},
};

pub fn shamir_secret_sharing(
    secret: BigInt,
    no_of_shares: BigInt,
    threshold: BigInt,
    prime_modulus: i32,
) -> Result<(), Box<dyn Error>> {
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

    let reconstructed_secret = match reconstruct_secret(&shares.shares, k_shares, threshold) {
        Ok(sec) => sec,
        Err(err) => {
            println!("{}", err);
            return Err(err);
        }
    };

    println!("Reconstructed Secret : {}\n", reconstructed_secret);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::secret_sharing::secret_splitting::Share;

    const PRIME_MODULUS: i32 = 97;

    #[test]
    fn test_split_and_reconstruct() {
        let secret = BigInt::from(65);
        let threshold = BigInt::from(3);
        let no_of_shares = BigInt::from(5);

        let polynomial: LagrangePolynomial = LagrangePolynomial::generate_polynomial(
            secret.clone(),
            threshold.clone(),
            PRIME_MODULUS,
        )
        .unwrap();

        let shares = Shares::generate_n_shares(&polynomial, no_of_shares).unwrap();

        let reconstructed_secret = reconstruct_secret(&shares.shares, 3, threshold).unwrap();

        assert_eq!(
            reconstructed_secret, secret,
            "Reconstructed secret does not match original!"
        );
    }

    #[test]
    fn test_insufficient_shares() {
        let secret = BigInt::from(65);
        let threshold = BigInt::from(3);
        let no_of_shares: BigInt = BigInt::from(5);

        let polynomial: LagrangePolynomial = LagrangePolynomial::generate_polynomial(
            secret.clone(),
            threshold.clone(),
            PRIME_MODULUS,
        )
        .unwrap();

        let shares = Shares::generate_n_shares(&polynomial, no_of_shares).unwrap();

        // Attempt to reconstruct with fewer than threshold share

        let k_shares = 2; // required threshold is 3 but we have only 2 shares.

        let reconstructed_secret = reconstruct_secret(&shares.shares, k_shares, threshold);

        assert!(
            reconstructed_secret.is_err(),
            "Secret reconstruction should fail with insufficient shares!"
        );
    }

    #[test]
    fn test_duplicate_shares() {
        let secret = BigInt::from(65);
        let threshold = BigInt::from(3);
        let no_of_shares: BigInt = BigInt::from(5);

        let polynomial: LagrangePolynomial = LagrangePolynomial::generate_polynomial(
            secret.clone(),
            threshold.clone(),
            PRIME_MODULUS,
        )
        .unwrap();

        let shares = Shares::generate_n_shares(&polynomial, no_of_shares).unwrap();

        let mut selected_shares: Vec<Share> = shares
            .shares
            .into_iter()
            .take(
                threshold
                    .to_usize()
                    .expect("Failed to convert BigInt to usize"),
            )
            .collect();

        // Alter one of the shares (e.g., change its y-coordinate)
        selected_shares[0].y += BigInt::from(1);

        let k_shares = 3;

        let reconstructed_secret =
            reconstruct_secret(&selected_shares, k_shares, threshold).unwrap();

        assert_ne!(
            reconstructed_secret, secret,
            "Reconstructed secret failed with duplicate shares!"
        );
    }
}
