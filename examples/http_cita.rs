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
