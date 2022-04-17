# Rust Web3 basics tutorial

This repository is a companion repository to my blog post about the basics of connecting to the Ethereum network using Rust and Web3. There is also an example of how to call functions on a smart contract.

The blog post can be found here: [Rust Web3 connect to Ethereum blockchain: how to](https://tms-dev-blog.com/rust-web3-connect-to-ethereum/).

## Set up and run

The project requires a .env file in the root directory to run properly. The .env file should contain the following:

```bash
INFURA_RINKEBY=wss://rinkeby.infura.io/ws/v3/xxxxxxx
ACCOUNT_ADDRESS=xxxxxxxxxx
```

The `INFURA_RINKEBY` value is an endpoint address from [infura.io](https://infura.io), however it can be any valid address to an Ethereum network WebSocket endpoint.

The `ACCOUNT_ADDRESS` is the address of a Ethereum wallet.

If this configuration is in place the program can be run using cargo:

```bash
cargo run
```
