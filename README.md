![OpenTron logo](https://github.com/opentron/press-kit/raw/master/logos/logo-w-typeface-white-1100x300.png)
[![Chat on Telegram](https://img.shields.io/badge/opentron-Chat%20on%20Telegram-blue)](https://t.me/opentron)

OpenTron is an implementation of the Tron blockchain written in Rust. This project is under active development and is
not ready for general use.

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->

- [Rationale](#rationale)
- [Roadmap](#roadmap)
  - [TODOs](#todos)
- [Quickstart](#quickstart)
- [License](#license)
  - [Contribution](#contribution)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

## Rationale

1. Decentralised

   The Tron network currently has [one and only implementation](https://github.com/tronprotocol/java-tron). This
   has lead to some criticism of Tron being too centralized. We want to change that by providing an alternative
   implementation and independent development team.

2. High performance

   API calls to java-tron nodes often results in CPU timeouts and other "out of resource" related errors. This is partly
   due to java-tron being written in Java, a garbage collected language that runs on a virtual machine. OpenTron is
   written in Rust, a modern compiled language that is increasingly adopted for blockchain and systems development due
   to its high performance, safety and modern design.

3. Modern codebase

   Java-tron was forked a few years ago from a Java Ethereum implementation. It has accumulated a lot of technical debt
   over the years and has a lot of inconsistent or missing documentation. We believe that a greenfield implementation
   will enable us to produce a cleaner code base that is easier to understand, improve and extend. In addition, since
   Rust has first class support for WebAssembly, it will be possible to re-use its code for creating web based clients,
   wallets, explorers, etc.

## Roadmap

- [x] Block data sync, only blocks (raw transactions), without transaction info and any other state data. Handle chain fork and block Merkle tree verification.
- [ ] Simple transaction broadcast, without much verification, just broadcast transactions to the network as quickly as possible(an airdrop tool can be made from it)
- [x] Handle transaction verification. all state data will be available. (difficult, EVM engine, resource consumption mode, witness/vote/proposal, chain parameter are all handled at this stage, to make the state data identical as java-tron )
- [x] Build a query API layer upon state data. json-rpc.
- [ ] Build a event API layer upon state data.
- [ ] block mining logic (difficult, DPoS mining, need resource to become an SR)

### TODOs

- [ ] Full Tron Protocol implementation
  - [x] proto2: refactor the protobuf
  - [x] primitives
    - [x] use primitive-types
    - [x] keys: Address, PublicKey, PrivateKey, Signature
  - [x] config file
    - [x] genesis block parsing
    - [x] toml config file parsing
    - [x] reorganize chain parameters
  - [x] chain-db
    - [x] Block / Transaction
  - [x] state-db
  - [x] discover protocol
  - [ ] channel protocol
    - [x] demo works
    - [x] sync
    - [ ] TODO: minor bug fix, timeout error
    - [ ] integrate with state-db
  - [ ] mempool - block producing
  - [x] governance
    - [x] witness schedule
    - [x] voting
    - [x] proposal
    - [ ] block reward - still has minor bug
  - [x] executor/actuator
    - [x] account
    - [x] trx/trc10 assets
    - [x] witness and voting
    - [x] proposal
    - [x] resource freeze/unfreeze
    - [x] exchange
    - [x] smart contract
    - [ ] market (4.1)
  - [ ] EVM / TVM
    - [x] 3.7 TVM <https://github.com/opentron/evm> and `tvm` crate
    - [x] 4.0 TVM with zksnark: `ztron` crate
      - [x] shielded trc20 transaction - ztron
      - [x] integration
    - [x] 4.1 TVM
      - [x] CREATE2
    - [ ] massive tests against resource usage, exit_reason
  - [ ] RPC API replacement
    - will not support gRPC
    - might have json-rpc support
    - [x] GraphQL API for chain query and state query
    - [ ] GraphQL API to broadcast transaction

## Quickstart

See [INSTALL.md](./INSTALL.md) for more detailed information.

```console
> # Install rust-nightly
> curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly
> rustup component add rustfmt

> # Install protoc
> brew install protobuf  # macOS
> sudo pacman -S protobuf  # ArchLinux
> sudo apt install protobuf-compiler libprotobuf-dev # Ubuntu / Debian

> # Get code
> git clone https://github.com/opentron/opentron.git

> # Compile opentron
> cd ./opentron/
> cargo build

> cargo run -- --config etc/conf.nile.toml
```

### Local Testnet

```sh
./target/release/opentron --config ./etc/conf.local-test.toml
```

Then open <http://localhost:3000> and play with the GraphQL APIs.

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
