# rust-web3

Ethereum JSON-RPC multi-transport client.
Rust implementation of Web3.js library.


# Examples
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
