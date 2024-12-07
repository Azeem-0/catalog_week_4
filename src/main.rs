pub mod sss;

use num_bigint::BigInt;
use secret_sharing_algorithms_sss_and_vss::{
    shamir_secret_sharing, verifiable_secret_sharing, SharingDetails,
};

fn main() {
    let sharing_details = SharingDetails {
        secret: BigInt::from(110),
        no_of_shares: BigInt::from(4),
        threshold: BigInt::from(2),
    };

    // shamir's secret sharing
    // shamir_secret_sharing(
    //     sharing_details.secret.clone(),
    //     sharing_details.no_of_shares.clone(),
    //     sharing_details.threshold.clone(),
    // );

    //verifiable secret sharing
    verifiable_secret_sharing(
        sharing_details.secret.clone(),
        sharing_details.no_of_shares.clone(),
        sharing_details.threshold.clone(),
        BigInt::from(5),
        BigInt::from(997),
    );
}
