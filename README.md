# codigo-anchor-program-unit-test
* Implement 100% Unit Test Coverage for Solana Native/Vanilla Programs and Anchor


# Solana NFT Minting, Transfer, and Burn Unit Tests

## Overview

This repository contains a set of comprehensive unit tests for the Solana-based NFT minting system (https://github.com/Codigo-io/platform/blob/develop/examples/solana_native/basic_nft/program/generated/processor.rs). The tests validate core functionalities, including minting, transferring, and burning NFTs on the Solana blockchain. It ensures that the core smart contract logic works as intended and handles edge cases securely.

The unit tests are built using `solana_program_test`, which allows testing in a simulated Solana blockchain environment, ensuring robustness and correctness before deploying the contract to the mainnet.

## Features Tested

### 1. Minting NFTs
The minting test (`test_mint`) verifies the successful creation of an NFT by interacting with a mint account. It simulates minting operations to ensure that:
- The mint instruction properly initializes the mint account.
- The ownership of the mint account is correctly set to the SPL Token program.
- Proper permissions are enforced during the minting process.

### 2. Transferring NFTs
The transfer test (`test_transfer`) simulates the transfer of an NFT from one account to another. It ensures:
- The NFT is correctly moved from the source account to the destination account.
- The source account no longer has ownership of the NFT.
- The destination account holds the expected data.

### 3. Burning NFTs
The burn test (`test_burn`) checks whether the NFT can be successfully burned, meaning:
- The associated mint account is removed from the blockchain after burning.
- Proper verification is done to ensure the account no longer exists.

### 4. Error Handling and Edge Cases
The following edge cases are tested to ensure the robustness of the system:

- **Invalid Signer Permission (`test_invalid_signer_permission`)**: Verifies that transactions fail when required signer permissions are missing.
- **Not Expected Address (`test_not_expected_address`)**: Ensures errors are returned if an unexpected Program Derived Address (PDA) is used.
- **Wrong Account Owner (`test_wrong_account_owner`)**: Checks that transactions with incorrect account owners are rejected.
- **Invalid Account Length (`test_invalid_account_len`)**: Ensures that accounts with incorrect data lengths trigger an appropriate error.

## Technology Stack

- **Rust Programming Language**: Used for implementing the smart contract logic and writing the unit tests.
- **Solana SDK**: Provides the tools to interact with Solana programs, accounts, and transactions.
- **Solana Program Test**: A testing framework to simulate the Solana blockchain environment, enabling unit testing for on-chain programs.

## Getting Started

### Prerequisites

- **Rust Toolchain**: Install the Rust programming language via [rustup](https://rustup.rs/).
- **Solana CLI**: Install the Solana CLI tools by following the [official guide](https://docs.solana.com/cli/install-solana-cli-tools).

### Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/your-username/solana-nft-tests.git
   cd solana-nft-tests
   ```
2. Install dependencies:
   ```bash
   cargo build
   ```

### Running Tests

To execute the unit tests, run the following command in the root directory of the project:
```bash
cargo test
```
This command will execute all tests defined in the module and provide detailed output, including any assertion errors and panic messages.

## Code Structure

- **Processor Module**: Implements the core logic of minting, transferring, and burning NFTs. This includes validating accounts, PDAs, and enforcing ownership rules.
- **Tests Module**: Contains all the unit tests for validating the core logic.
  - `test_mint()`: Tests minting functionality.
  - `test_transfer()`: Tests the NFT transfer process.
  - `test_burn()`: Tests the burn operation.
  - `test_invalid_signer_permission()`: Tests scenarios with missing signers.
  - `test_not_expected_address()`: Tests for invalid PDAs.
  - `test_wrong_account_owner()`: Tests the case where the account owner is incorrect.
  - `test_invalid_account_len()`: Tests for errors when incorrect data lengths are provided.

## Important Considerations

- **Security**: These tests ensure that ownership, permissions, and PDA validations are correctly enforced. Proper error handling is critical for preventing unauthorized actions.
- **Simulated Environment**: All tests are performed using the Solana program test framework, which emulates a real blockchain environment for testing purposes.
- **Error Handling**: Each test includes scenarios where improper usage should trigger an error. This helps validate that the program handles errors as expected.

## Contribution Guidelines

We welcome contributions to improve these tests or add new features! Please follow the steps below:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature-branch-name`).
3. Commit your changes (`git commit -am 'Add new feature'`).
4. Push to the branch (`git push origin feature-branch-name`).
5. Open a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.


