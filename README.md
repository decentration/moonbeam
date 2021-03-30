# ![Moonbeam](media/moonbeam-cover.jpg)

![Tests](https://github.com/PureStake/moonbeam/workflows/Release/badge.svg)

An Ethereum compatible ~~[Parachain](https://polkadot.network/technology/)~~ built with [Substrate](https://substrate.dev)

_Discover the Moonbeam project at [moonbeam.network](https://moonbeam.network)._
_Learn to [use the Moonbeam network](https://docs.moonbeam.network/) with our technical docs._
_Reference our [crate-level docs (rustdocs)](https://purestake.github.io) to contribute._

## Run an alphanet node with Docker

Docker images are published for every tagged release. Learn more with `moonbeam --help`.

```bash
# Join the public testnet
docker run --network="host" purestake/moonbeam:v0.6.1 --chain alphanet
```

## Run a local development node with Docker

Developers who are building dApps to run on moonbeam, may want a lightweight node to work with
locally. You can quickly spin up a single node with no relay chain backing it using the development
service.

```bash
# Run a dev service node.
docker run --network="host" purestake/moonbeam:v0.6.1 --dev
```

### Sealing options

The command above will start the node in instant seal mode. It creates a block when a transaction arrives, similar to Ganache's auto-mine. You can also choose to author blocks at a regular interval, or control authoring manually through the RPC.

```bash
# Author a block every 6 seconds.
docker run --network="host" purestake/moonbeam:v0.6.1 --dev --sealing 3000

# Manually control the block authorship and finality
docker run --network="host" purestake/moonbeam:v0.6.1 --dev --sealing manual
```

### Dev Addresses

Launching the node in devmode will prefund a list of dev addresses that are derived from
the canonical mnemonic: "bottom drive obey lake curtain smoke basket hold race lonely fit walk"

#### Alith:

- Address:0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac
- PrivKey:0x5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133

#### Baltathar:

- Address:0x3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0
- PrivKey:0x8075991ce870b93a8870eca0c0f91913d12f47948ca0fd25b49c6fa7cdbeee8b

#### Charleth:

- Address:0x798d4Ba9baf0064Ec19eB4F0a1a45785ae9D6DFc
- PrivKey:0x0b6e18cafb6ed99687ec547bd28139cafdd2bffe70e6b688025de6b445aa5c5b

#### Dorothy:

- Address:0x773539d4Ac0e786233D90A233654ccEE26a613D9
- PrivKey:0x39539ab1876910bbf3a223d84a29e28f1cb4e2e456503e7e91ed39b2e7223d68

#### Ethan:

- Address:0xFf64d3F6efE2317EE2807d223a0Bdc4c0c49dfDB
- PrivKey:0x7dce9bc8babb68fec1409be38c8e1a52650206a7ed90ff956ae8a6d15eeaaef4

#### Faith:

- Address:0xC0F0f4ab324C46e55D02D0033343B4Be8A55532d
- PrivKey:0xb9d2ea9a615f3165812e8d44de0d24da9bbd164b65c4f0573e1ce2c8dbd9c8df

#### Gerald:

- Address:0x7BF369283338E12C90514468aa3868A551AB2929
- PrivKey:0x96b8a38e12e1a31dee1eab2fffdf9d9990045f5b37e44d8cc27766ef294acf18

## Build the Moonbeam Node

To build Moonbeam, you will need a proper Substrate development environment. If you've never worked
with a Substrate-based blockchain before, you should probably try the [Setting Up a Moonbeam Node]
(https://docs.moonbeam.network/getting-started/local-node/setting-up-a-node/) docs first. If you
need a refresher setting up your Substrate environment, see [Substrate's Getting Started Guide]
(https://substrate.dev/docs/en/knowledgebase/getting-started/).

```bash
# Fetch the code
git clone https://github.com/PureStake/moonbeam
cd moonbeam

# Build the node (The first build will be long (~30min))
cargo build --release
```

## Run tests

Moonbeam has Rust unit tests as well as typescript integration tests.

```bash
# Run the Rust unit tests
cargo test
```

```bash
# Install dependencies for integration tests
cd moonbeam-types-bundle
npm i

cd ../tests
npm i

# Run integration tests
npm test
```

## Chain IDs

The Ethereum specification described a numeric Chain Id. The Moonbeam mainnet Chain Id will be 1284
because it takes 1284 milliseconds for a moonbeam to reach Earth.

Moonbeam nodes support multiple public chains and testnets, with the following Chain Ids.

| Network Description                | Chain ID    |
| ---------------------------------- | ----------- |
| Local Parachain TestNet            | 1280        |
| Local Development TestNet          | 1281        |
| Reserved for other TestNets        | 1282 - 1283 |
| Moonbeam (Polkadot)                | 1284        |
| Moonriver (Kusama)                 | 1285        |
| Moonrock (Rococo)                  | 1286        |
| Moonbase Alpha TestNet             | 1287        |
| Reserved for other public networks | 1288 - 1289 |

## Runtime Architecture

The Moonbeam Runtime is built using FRAME and consists of pallets from substrate, frontier, cumulus, and `pallets/`.

From substrate:

- _Utility_: Allows users to use derivative accounts, and batch calls
- _Balances_: Tracks GLMR token balances
- _Sudo_: Allows a privileged account to make arbitrary runtime changes - will be removed before
  launch
- _Timestamp_: On-Chain notion of time
- _Transaction Payment_: Transaction payment (fee) management
- _Randomness Collective Flip_: A (mock) onchain randomness beacon. Will be replaced by parachain
  randomness by mainnet.

From frontier:

- _EVM_: Encapsulates execution logic for an Ethereum Virtual Machine
- _Ethereum_: Ethereum-style data encoding and access for the EVM.

From cumulus:

- _ParachainUpgrade_: A helper to perform runtime upgrades on parachains
- _ParachainInfo_: A place to store parachain-relevant constants like parachain id

The following pallets are stored in `pallets/`. They are designed for Moonbeam's specific requirements:

- _Ethereum Chain Id_: A place to store the chain id for each Moonbeam network
- _Author Inherent_: Allows block authors to include their identity in a block via an inherent
- _Parachain Staking_: Minimal staking pallet that selects collators by total amount at stake

## Tests

Tests are run with the following command:

```bash
cargo test --verbose
```

This github repository is also linked to Gitlab CI

## Contribute

Moonbeam is open source under the terms of the GPL3. We welcome contributions. You can explore our
crate-level documentation at https://purestake.github.io/moonbeam

### Code style

Moonbeam is following the
[Substrate code style](https://github.com/paritytech/substrate/blob/master/docs/STYLE_GUIDE.md)  
We provide a [.editorconfig](.editorconfig) (_compatible with VSCode using RLS_)