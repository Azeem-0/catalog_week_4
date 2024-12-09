pub mod sss;
pub mod vss;

pub mod secret_sharing;

use num_bigint::BigInt;
use sss::shamir_secret_sharing;
use vss::verifiable_secret_sharing;

pub struct SecretSharingParams {
    pub secret: BigInt,
    pub no_of_shares: BigInt,
    pub threshold: BigInt,
    pub prime_modulus: i32,
}

fn main() {
    let secret_sharing_params = SecretSharingParams {
        secret: BigInt::from(65),
        no_of_shares: BigInt::from(5),
        threshold: BigInt::from(3),
        prime_modulus: 97,
    };

    println!(
        "secret : {} , no of shares : {} threshold : {}, prime modulus : {}",
        secret_sharing_params.secret,
        secret_sharing_params.no_of_shares,
        secret_sharing_params.threshold,
        secret_sharing_params.prime_modulus
    );

    println!("\n");
    println!("Implementing shamir's secret sharing algorithm\n");

    match shamir_secret_sharing(
        secret_sharing_params.secret.clone(),
        secret_sharing_params.no_of_shares.clone(),
        secret_sharing_params.threshold.clone(),
        secret_sharing_params.prime_modulus.clone(),
    ) {
        Ok(()) => println!("Successfully implemented shamir's secret sharing"),
        Err(err) => println!("{}", err),
    };

    println!("\n");
    println!("Implementing feldman's verifiable secret sharing algorithm\n");

    match verifiable_secret_sharing(
        secret_sharing_params.secret,
        secret_sharing_params.no_of_shares,
        secret_sharing_params.threshold,
        secret_sharing_params.prime_modulus,
    ) {
        Ok(()) => println!("Successfully implemented verifiable secret sharing"),
        Err(err) => println!("{}", err),
    };
}
