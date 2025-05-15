# Solana PoC Program using Anchor Framework

## Installation

### Install Solana CLI

[Solana Installation Docs](https://docs.anza.xyz/cli/install/)

```bash
sh -c "$(curl -sSfL https://release.anza.xyz/v1.18.17/install)"
```

### Install Anchor Framework

[Anchor Installation Docs](https://www.anchor-lang.com/docs/installation)

```bash
cargo install --git https://github.com/coral-xyz/anchor avm --force
avm use 0.29.0
```

### Install NodeJS & Yarn

Use `nvm` to install NodeJS v20 and `npm` to install Yarn:

```bash
nvm install 20
nvm use 20
npm i -g yarn
```

### Sanity Check


```bash
solana --version
avm --version
anchor --version
rustc --version
node --version
```

On 'my machine' ;) it yeilds:

```
solana-cli 1.18.17 (src:c027cfc3; feat:4215500110, client:Agave)
avm 0.30.1
anchor-cli 0.29.0
rustc 1.79.0 (129f3b996 2024-06-10)
v20.19.2
```

## Build & Test

To build and test from a fresh clone:

```bash
yarn
anchor build
anchor test
```

## How this program was originally setup

The following commands were used to initialte this repo

```
avm use 0.29.0
nvm use 20
anchor init solana_poc
cd solana_poc
nvm use 20
anchor build
solana config set --url localhost
anchor keys sync
anchor test
```