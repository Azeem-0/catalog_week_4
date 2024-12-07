use num_bigint::BigInt;

use crate::secret_sharing::{
    secret_reconstruction::reconstruct_secret,
    secret_splitting::{LagrangePolynomial, Shares},
};

const PRIME_MODULUS: i32 = 997;

pub fn shamir_secret_sharing(secret: BigInt, no_of_shares: BigInt, threshold: BigInt) {
    let polynomial: LagrangePolynomial =
        LagrangePolynomial::generate_polynomial(secret, threshold.clone(), PRIME_MODULUS).unwrap();

    println!("{:?}", polynomial);

    let shares = Shares::generate_n_shares(&polynomial, no_of_shares).unwrap();

    let reconstructed_secret = reconstruct_secret(&shares.shares, 3, threshold).unwrap();

    println!("Reconstructed Secret : {}", reconstructed_secret);
}
