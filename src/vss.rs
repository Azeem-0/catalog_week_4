use num::BigInt;

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
