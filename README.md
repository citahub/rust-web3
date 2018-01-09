# rust-web3

Ethereum and CITA JSON-RPC multi-transport client.
Rust implementation of Web3.js library.


# Examples1
```rust
extern crate tokio_core;
extern crate web3;

use web3::futures::Future;
use web3::types::{BlockId, BlockNumber};

const MAX_PARALLEL_REQUESTS: usize = 64;

fn main() {
    let mut event_loop = tokio_core::reactor::Core::new().unwrap();
    let web3 = web3::Web3::new(
        web3::transports::Http::with_event_loop(
            "http://localhost:1337",
            &event_loop.handle(),
            MAX_PARALLEL_REQUESTS,
        ).unwrap(),
    );

    //get height
    let block_number = web3.cita().block_number().map(|height| {
        println!("height: {:?}", height);
    });
    event_loop.run(block_number).unwrap();

    //get peer count
    let peer_count = web3.net().peer_count().map(|peer_count| {
        println!("peer_count: {:?}", peer_count);
    });
    event_loop.run(peer_count).unwrap();

    //get block
    let block = web3.cita()
        .block(BlockId::Number(BlockNumber::Latest))
        .map(|block| {
            println!("block: {:?}", block);
        });
    event_loop.run(block).unwrap();
}
```

# example2: send_transaction
For more create contract information, you can see jsonrpc [readme.md](https://github.com/cryptape/cita/blob/develop/cita-jsonrpc/READE.md) 
```rust
extern crate cita_crypto;
extern crate protobuf;
extern crate rustc_hex;
extern crate tokio_core;
extern crate web3;

use cita_crypto::*;
use web3::futures::Future;

use protobuf::core::Message;
use rustc_hex::ToHex;

const MAX_PARALLEL_REQUESTS: usize = 64;
const CONTRUCT_CODE: &str = "60606040523415600e57600080fd5b5b5b5b60948061001f6000396000f300\
                             60606040526000357c01000000000000000000000000000000000000000000\
                             00000000000000900463ffffffff1680635524107714603d575b600080fd5b\
                             3415604757600080fd5b605b6004808035906020019091905050605d565b00\
                             5b806000819055505b505600a165627a7a72305820c471b4376626da2540b2\
                             374e8b4110501051c426ff46814a6170ce9e219e49a80029";

fn main() {
    let mut event_loop = tokio_core::reactor::Core::new().unwrap();
    let web3 = web3::Web3::new(
        web3::transports::Http::with_event_loop(
            "http://localhost:1337",
            &event_loop.handle(),
            MAX_PARALLEL_REQUESTS,
        ).unwrap(),
    );

    //study create sendtransaction param
    let cita = web3.cita();
    let height = cita.block_number().map(|height| {
        println!("height: {:?}", height);
        height
    });

    let number = event_loop.run(height).unwrap();
    let key_pair = KeyPair::gen_keypair();

    //create contract
    let number = number.low_u64();
    println!("number: {:?}", number);
    let tx = cita.generate_tx(
        key_pair.privkey(),
        CONTRUCT_CODE.to_string(),
        "".to_string(),
        number,
        2500,
        "abcd".to_string(),
    ).write_to_bytes()
        .unwrap()
        .to_hex();
    println!("hex = {:?}", tx);
    let tx = cita.send_transaction(tx).map(|tx_response| {
        println!("tx_response: {:?}", tx_response);
        tx_response
    });
    event_loop.run(tx).unwrap();
}

```

If you want to deploy smart contracts you have written you can do something like this (make sure you have the solidity compiler installed):

`solc -o build --bin --abi contracts/*.sol`

The solidity compiler is generating the binary and abi code for the smart contracts in a directory called contracts and is being output to a directory called build.

For more see [examples folder](./examples)

# TODO

## General
- [ ] More flexible API (accept `Into<X>`)
- [x] Contract calls (ABI encoding; `debris/ethabi`)
- [ ] Batch Requests

## Transports
- [x] HTTP transport
- [x] IPC transport
- [ ] WebSockets transport

## Types
- [x] Types for `U256,H256,Address(H160)`
- [x] Index type (numeric, encoded to hex)
- [x] Transaction type (`Transaction` from Parity)
- [x] Transaction receipt type (`TransactionReceipt` from Parity)
- [x] Block type (`RichBlock` from Parity)
- [x] Work type (`Work` from Parity)
- [ ] Syncing type (`SyncStats` from Parity)

## APIs
- [x] Eth: `eth_*`
- [x] Eth filters: `eth_*`
- [x] `net_*`
- [x] `web3_*`
- [x] `personal_*`
- [x] `cita_*`
- [ ] `traces_*`

### Parity-specific APIs
- [ ] Parity read-only: `parity_*`
- [ ] Parity accounts: `parity_*`
- [ ] Parity set: `parity_*`
- [ ] `signer_*`

- [x] Own APIs (Extendable)
```rust
let web3 = Web3::new(transport);
web3.api::<CustomNamespace>().custom_method().wait().unwrap()
```

# Installation on Windows

Currently, Windows does not support IPC, which is enabled in the library by default.
To complile, you need to disable IPC feature:
```
web3 = { version = "0.1.0", default-features = false, features = ["http"] }
```
