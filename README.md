
# Nexis Network - Readme

## Chain Specifications

- **Native Token**: Nexis (NZT)
- **Symbol**: NZT
- **Total Supply**: 550 Million
- **Block Finality**: Approx. 400ms
- **Consensus Mechanism**: Delegated Proof-of-Stake (DPoS)
- **Networking Ports**: P2P (TBD), JSON-RPC (8899)
- **Chain IDs**: Mainnet (1229), Testnet (2370)
- **EVM Compatibility**: Fully compatible with Ethereum Virtual Machine

## Official Resources

- **Website**: [nexis.network](https://nexis.network)
- **Blockchain Explorers**: [Mainnet Explorer](https://evm.nexiscan.io), [Testnet Explorer](https://testnet.nexiscan.io)
- **RPC Endpoints**: [Mainnet](https://rpc-main-1.nexis.network), [Testnet](https://rpc-test-1.nexis.network), 
- **Social Media**: [Twitter](https://twitter.com/exzo_network), [Telegram](https://t.me/Nexis_Network)
- **Testnet Faucet**: [Faucet](https://evm-faucet.nexis.network)

## Key Features

- Modular architecture for scalability and adaptability.
- Ethereum compatibility allows interaction with Ethereum wallets and tools via JSON-RPC.
- Development support in Solidity/Vyper with full EVM integration.
- Streamlined cross-chain operations for improved user experience.
- Extend beyond Ethereum's capabilities with Runtime plugins.

## NZT Token Utilities

- **Network Fees**: Used to pay transaction and stability fees.
- **Governance**: Token holders can vote on network proposals.
- **Economic Capital**: Utilized in liquidations.
- **Discounts**: Fee reductions on DApps and wallets.
- **Staking**: Secure the network through staking.

## Cross-Chain Communication

- Trustless, decentralized Ethereum bridge for asset transfers.
- Supports ERC-20 tokens, NFTs, and wrapped currencies.
- Customizable bridge functionalities with plugins.

## Development Guide

### Environment Setup

Follow these steps to set up your development environment:

1. **Install Rust and Dependencies**:

   - Run the following command to install Rust and its dependencies:
     ```bash
     curl https://sh.rustup.rs -sSf | sh
     source $HOME/.cargo/env
     rustup component add rustfmt
     rustup update
     ```

   - Additional Linux dependencies (Ubuntu):
     ```bash
     sudo apt-get update
     sudo apt-get install libssl-dev libudev-dev pkg-config zlib1g-dev llvm clang make cmake protobuf-compiler
     ```

   - On Mac M1, install Rosetta:
     ```bash
     softwareupdate --install-rosetta
     ```

2. **Download and Build Source Code**:

   - Clone the repository:
     ```bash
     git clone https://github.com/Nexis-Network/NEXIS.git
     cd nexis
     ```

   - Build the source code:
     ```bash
     cargo build
     ```

3. **Run a Local Cluster**:

   - Start a local cluster by running the following command:
     ```bash
     ./run.sh
     ```

### Testing and Benchmarking

- **Run Tests**:

  - Execute the following command to run tests:
    ```bash
    cargo test --no-fail-fast
    ```

- **EVM Integration and Local Testnet**:

  - Refer to the documentation at [docs.nexis.network/evm](https://docs.nexis.network/evm) for information on EVM integration and setting up a local testnet.

- **Benchmarking**:

  - Install the nightly Rust build:
    ```bash
    rustup install nightly
    ```

  - Execute benchmarks:
    ```bash
    cargo +nightly bench
    ```

### Troubleshooting

If you encounter any issues during the setup or development process, consider the following troubleshooting steps:

- Make sure you have installed all the required dependencies as mentioned in the environment setup section.

- Check if your Rust installation is up to date by running:
  ```bash
  rustup update
  ```

- If you are facing issues with building the source code, ensure that you have the necessary build tools and libraries installed.

- For any specific errors or problems, refer to the project's issue tracker or community forums for assistance.

## Release and Copyright

- **Release Process**: Described in [RELEASE.md](RELEASE.md).
- **Copyright**: 
  ```
  Copyright 2023-2024 Nexis Network
  Licensed under the Apache License, Version 2.0.
  Full license at http://www.apache.org/licenses/LICENSE-2.0
  ```
