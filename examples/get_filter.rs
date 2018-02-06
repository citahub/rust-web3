extern crate tokio_core;
extern crate web3;

use web3::futures::Future;
use web3::types::FilterBuilder;
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

    let filter = web3.eth_filter();
    let filter_build = FilterBuilder::default().limit(10).build();
    let filter_topic = filter.create_logs_filter(filter_build);

    let log = event_loop.run(filter_topic).unwrap();
    println!("log = {:?}", log);
}
