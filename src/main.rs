pub mod sss;

use num_bigint::BigInt;
use secret_sharing_algorithms_sss_and_vss::{shamir_secret_sharing, verifiable_secret_sharing};

fn main() {
    // shamir's secret sharing
    shamir_secret_sharing(BigInt::from(564), BigInt::from(5), BigInt::from(3));

    //verifiable secret sharing
    verifiable_secret_sharing();
}
