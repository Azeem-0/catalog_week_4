pub mod sss;
pub mod vss;
use num_bigint::BigInt;
use sss::{generate_n_shares, generate_polynomial, reconstruct_secret, Polynomial};
use vss::Commitments;

pub struct SharingDetails {
    pub secret: BigInt,
    pub no_of_shares: BigInt,
    pub threshold: BigInt,
}

pub fn shamir_secret_sharing(secret: BigInt, no_of_shares: BigInt, threshold: BigInt) {
    println!("Implementing shamir's secret sharing.");

    println!(
        "secret : {} , no of shares : {} threshold : {}",
        secret, no_of_shares, threshold
    );

    let polynomial: Polynomial = generate_polynomial(secret, threshold.clone()).unwrap();

    println!("{:?}", polynomial);

    let shares = generate_n_shares(&polynomial, no_of_shares).unwrap();

    println!("{:?}", shares);

    let reconstructed_secret = reconstruct_secret(&shares.shares, 3, threshold).unwrap();

    println!("Reconstructed Secret : {}", reconstructed_secret);
}

pub fn verifiable_secret_sharing(
    secret: BigInt,
    no_of_shares: BigInt,
    threshold: BigInt,
    generator: BigInt,
    prime_modulus: BigInt,
) {
    println!("Implementing verifiable secret sharing.");

    println!(
        "secret : {} , no of shares : {} threshold : {} generator {} prime modulus : {}",
        secret, no_of_shares, threshold, generator, prime_modulus
    );

    let polynomial: Polynomial = generate_polynomial(secret, threshold.clone()).unwrap();

    println!("{:?}", polynomial);

    let shares = generate_n_shares(&polynomial, no_of_shares).unwrap();

    println!("{:?}", shares);

    let commitments: Commitments =
        Commitments::generate_commitments(&generator, &polynomial).unwrap();

    println!("{:?}", commitments);

    for share in &shares.shares {
        let validation = Commitments::verify_share(&share.x, &share.y, &generator, &commitments);

        println!("The share is : {}", validation);
    }

    let reconstructed_secret = reconstruct_secret(&shares.shares, 3, threshold).unwrap();

    println!("{}", reconstructed_secret);
}
