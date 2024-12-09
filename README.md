# Secret Sharing Scheme (SSS) and Verifiable Secret Sharing (VSS)
## Introduction 

Secret sharing is a cryptographic technique that ensures secure distribution of sensitive data among multiple participants. By dividing a secret into parts, the original secret can only be reconstructed with a specified minimum number of shares, providing both security and redundancy. Applications include secure key management, distributed trust systems, and blockchain technologies.

## Overview

This project implements **Shamir's Secret Sharing** (SSS) and **Verifiable Secret Sharing** (VSS) algorithms using **Lagrange interpolation**. The goal is to divide a secret into multiple shares and provide the ability to reconstruct the secret using a subset of those shares. The project also supports verifying the correctness of shares during reconstruction.

## Project Goals 

- Demonstrate the implementation of secure and efficient SSS and VSS algorithms.
- Explore error handling in secret reconstruction.
- Ensure that shares are verifiable before reconstruction.

## Key Features:
- **Shamir's Secret Sharing (SSS)**: Divides a secret into multiple shares such that a threshold number of shares are required to reconstruct the secret.
- **Verifiable Secret Sharing (VSS)**: Extends SSS by allowing the verification of shares before using them in reconstruction.
- **Rust Implementation**: Written in Rust for efficiency and safety.


## Prerequisites

- **Rust**: Make sure you have the latest stable version of Rust installed.
- **Cargo**: Comes pre-installed with Rust.


## Installation

To install follow the steps below:

1. **Clone the Repository**:
    ```sh
    git clone https://github.com/Azeem-0/secret_sharing_algorithms_sss_and_vss.git
    ```

2. **Install Dependencies**:
    Use `Cargo` (Rust's package manager), it will automatically fetch the dependencies when building the project.

3. **Build the Project**:
    ```sh
    cargo build 
    ```

4. **Run the Tests**:
    To verify that everything is working correctly:
    ```sh
    cargo test
    ```
5. **Running the project**:
    To run the project : 
    ```bash
    cargo run
    ```

## Test Coverage 

- Unit tests for SSS and VSS functions.
- Edge case tests (e.g., handling insufficient shares, invalid shares, verifying shares, verifying invalid shares).
