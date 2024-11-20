# Building a blockchain with Polkadot SDK

* 🤏 This template is a minimal (in terms of complexity and the number of components)
template for building a blockchain node.

* 🔧 Its runtime is configured with a single custom pallet as a starting point, and a handful of ready-made pallets
such as a [Balances pallet](https://paritytech.github.io/polkadot-sdk/master/pallet_balances/index.html).

* 👤 The template has no consensus configured - it is best for experimenting with a single node network.

## Challenge Description

TBA

## Template Structure

A Polkadot SDK based project such as this one consists of:

* 💿 a [Node](./node/README.md) - the binary application.
* 🧮 the [Runtime](./runtime/README.md) - the core logic of the blockchain.
* 🎨 the [Pallets](./pallets/README.md) - from which the runtime is constructed.

## Getting Started

* 🦀 The template is using the Rust language.

* 👉 Check the
[Rust installation instructions](https://www.rust-lang.org/tools/install) for your system.

* 🛠️ Depending on your operating system and Rust version, there might be additional
packages required to compile this template - please take note of the Rust compiler output.

### Build

🔨 Use the following command to build the node without launching it:

```sh
cargo build --release
```

🐳 Alternatively, build the docker image:

```sh
docker build . -t polkadot-sdk-minimal-template
```

### Single-Node Development Chain

👤 The following command starts a single-node development chain:

```sh
./target/release/minimal-template-node --dev

# docker version:
docker run --rm polkadot-sdk-minimal-template --dev
```

Development chains:

* 🧹 Do not persist the state.
* 💰 Are pre-configured with a genesis state that includes several pre-funded development accounts.
* 🧑‍⚖️ One development account (`ALICE`) is used as `sudo` accounts.
