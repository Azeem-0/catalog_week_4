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
    pub generator: BigInt,
}

fn main() {
    let secret_sharing_params = SecretSharingParams {
        secret: BigInt::from(65),
        no_of_shares: BigInt::from(4),
        threshold: BigInt::from(3),
        generator: BigInt::from(5),
    };

    println!(
        "secret : {} , no of shares : {} threshold : {}, generator : {}",
        secret_sharing_params.secret,
        secret_sharing_params.no_of_shares,
        secret_sharing_params.threshold,
        secret_sharing_params.generator
    );

    println!("\n");
    println!("Implementing shamir's secret sharing algorithm");

    shamir_secret_sharing(
        secret_sharing_params.secret.clone(),
        secret_sharing_params.no_of_shares.clone(),
        secret_sharing_params.threshold.clone(),
    );

    println!("\n");
    println!("Implementing feldman's verifiable secret sharing algorithm");

    verifiable_secret_sharing(
        secret_sharing_params.secret.clone(),
        secret_sharing_params.no_of_shares.clone(),
        secret_sharing_params.threshold.clone(),
        BigInt::from(5),
    );
}
