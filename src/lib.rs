pub mod sss;
use num_bigint::BigInt;
use sss::{generate_n_shares, generate_polynomial, reconstruct_secret, Polynomial};

pub fn shamir_secret_sharing(secret: BigInt, no_of_shares: BigInt, threshold: BigInt) {
    println!("Implementing shamir's secret sharing.");

    println!(
        "secret : {} , no of shares : {} threshold : {}",
        secret, no_of_shares, threshold
    );

    let polynomial: Polynomial = generate_polynomial(secret, threshold.clone()).unwrap();

    println!("{:?}", polynomial);

    let shares = generate_n_shares(polynomial, no_of_shares).unwrap();

    println!("{:?}", shares);

    let reconstructed_secret = reconstruct_secret(&shares.shares, 3, threshold).unwrap();

    println!("{}", reconstructed_secret);
}

pub fn verifiable_secret_sharing() {
    println!("Implementing verifiable secret sharing.");
}
