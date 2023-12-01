# Exzo Network
**Chain Specs**
* Native Token Name - Nexis
* Symbol - NZT
* Supply - 420 Million
* Block finality - ~400ms
* Consensus - Delegated Proof-of-Stake (DPoS)
* P2P Port - 
* JSON-RPC Port - 8899
* ChainID Mainnet - 1229
* ChainID Testnet -  2370
* Ethereum Virtual Machine (EVM) Compatible

**Official Links**
* Website - https://nexis.network
* Main Explorer - https://nexscan.io
* Test Explorer - https://testnet.exzoscan.io
* RPC Mainnet - https://rpc-main-1.nexis.network
* RPC Testnet - https://rpc-test-1.nexis.network
* Twitter - https://twitter.com/exzo_network
* Telegram - https://t.me/Nexis_Network

## About the Nexis Network 
* Nexis Network uses a modular architecture and provides Ethereum compatibility.
* Users able to interact with industry standard wallets through JSON-RPC.
* Develop with Solidity/Vyper, full EVM support.
* Build using widely-adopted Ethereum tooling, libraries and development tools.
* Streamlined UX when doing cross-chain operations.
* Go beyond Ethereum’s Smart Contracts with Runtime plugins.

The Nexis Network Token (NZT) features the following utilities, and the value of NZT token will accrue with the increased usage of the network and revenue from stability fees and liquidation penalties.

* As Network Utility Token: to pay for network fees and stability fees.
* As Governance Token: to vote for/against risk parameters and network change proposals.
* As Economic Capital: in case of liquidation without sufficient collaterals.
* As fee reducing token on decentralized applications and wallets upon the Exzo Network.
* For staking to help secure the Nexis Network in a decentralized fashion.

Cross-Chain Communication
* Completely trustless and decentralized built-in Ethereum Bridge solution.
* Transfer assets to and from any EVM compatible network, most notably Nexis Network and Ethereum mainnets.
* Transfer ERC-20 tokens, NFTs or wrapped native currencies.
* Customize the bridge functionality using Bridge plugins.

## Building

### **1. Install rustc, cargo and rustfmt.**

```bash
$ curl https://sh.rustup.rs -sSf | sh
$ source $HOME/.cargo/env
$ rustup component add rustfmt
```

Please sure you are always using the latest stable rust version by running:

```bash
$ rustup update
```

On Linux systems you may need to install libssl-dev, pkg-config, zlib1g-dev, etc.  On Ubuntu:

```bash
$ sudo apt-get update
$ sudo apt-get install libssl-dev libudev-dev pkg-config zlib1g-dev llvm clang make cmake protobuf-compiler
```

On Mac M1s, make sure you set up your terminal & homebrew [to use](https://5balloons.info/correct-way-to-install-and-use-homebrew-on-m1-macs/) Rosetta. You can install it with:

```bash
$ softwareupdate --install-rosetta
```

### **2. Download the source code.**

```bash
$ git clone https://github.com/ExzoNetwork/exzocoin.git
$ cd exzocoin
```

### **3. Build.**

```bash
$ cargo build
```

### **4. Run a minimal local cluster.**
```bash
$ ./run.sh
```

## Testing

**Run the test suite:**

```bash
$ cargo test --no-fail-fast
```

#### EVM integration
Info about EVM integration is at our [docs](https://docs.exzo.technology/evm).

#### Starting a local testnet
Start your own Development network locally, instructions are in the [online docs](https://docs.exzo.technology/cluster/bench-tps).

#### Accessing the remote testnet and mainnet
* `testnet` - public accessible via bootstrap.rpc-test-1.nexis.network.
* `mainnet` - public accessible via bootstrap.exzo.technology.

## Benchmarking

First install the nightly build of rustc. `cargo bench` requires use of the
unstable features only available in the nightly build.

```bash
$ rustup install nightly
```

Run the benchmarks:

```bash
$ cargo +nightly bench
```

## Release Process

The release process for this project is described [here](RELEASE.md).

## Copyright
---
```
Copyright 2022-2023 Exzo Network

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```
---
