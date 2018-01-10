//! Ethereum JSON-RPC client (Web3).

extern crate arrayvec;
extern crate cita_crypto;
#[macro_use]
extern crate error_chain;
extern crate ethabi;
extern crate jsonrpc_core as rpc;
#[macro_use]
extern crate log;
extern crate parking_lot;
extern crate protobuf;
extern crate rustc_hex;
extern crate rustc_serialize;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[cfg_attr(test, macro_use)]
extern crate serde_json;
extern crate tokio_timer;
extern crate util;

extern crate jsonrpc_types;
extern crate libproto;

/// Re-export of the `futures` crate.
#[macro_use]
pub extern crate futures;

// it needs to be before other modules
// otherwise the macro for tests is not available.
#[macro_use]
pub mod helpers;
pub mod api;
pub mod contract;
pub mod error;
pub mod transports;
pub mod types;
pub mod confirm;
pub mod cita_types;

use futures::Future;

pub use error::{Error, ErrorKind};
pub use api::Web3;

/// RPC result
pub type Result<T> = Box<Future<Item = T, Error = Error> + Send + 'static>;

/// Assigned RequestId
pub type RequestId = usize;

/// Transport implementation
pub trait Transport: ::std::fmt::Debug + Clone {
    /// The type of future this transport returns when a call is made.
    type Out: futures::Future<Item = rpc::Value, Error = Error>;

    /// Prepare serializable RPC call for given method with parameters.
    fn prepare(&self, method: &str, params: Vec<rpc::Value>) -> (RequestId, rpc::Call);

    /// Execute prepared RPC call.
    fn send(&self, id: RequestId, request: rpc::Call) -> Self::Out;

    /// Execute remote method with given parameters.
    fn execute(&self, method: &str, params: Vec<rpc::Value>) -> Self::Out {
        let (id, request) = self.prepare(method, params);
        self.send(id, request)
    }
}

/// A transport implementation supporting batch requests.
pub trait BatchTransport: Transport {
    /// The type of future this transport returns when a call is made.
    type Batch: futures::Future<Item = Vec<::std::result::Result<rpc::Value, Error>>, Error = Error>;

    /// Sends a batch of prepared RPC calls.
    fn send_batch<T>(&self, requests: T) -> Self::Batch
    where
        T: IntoIterator<Item = (RequestId, rpc::Call)>;
}

impl<X, T> Transport for X
where
    T: Transport + ?Sized,
    X: ::std::ops::Deref<Target = T>,
    X: ::std::fmt::Debug,
    X: Clone,
{
    type Out = T::Out;

    fn prepare(&self, method: &str, params: Vec<rpc::Value>) -> (RequestId, rpc::Call) {
        (**self).prepare(method, params)
    }

    fn send(&self, id: RequestId, request: rpc::Call) -> Self::Out {
        (**self).send(id, request)
    }
}

impl<X, T> BatchTransport for X
where
    T: BatchTransport + ?Sized,
    X: ::std::ops::Deref<Target = T>,
    X: ::std::fmt::Debug,
    X: Clone,
{
    type Batch = T::Batch;

    fn send_batch<I>(&self, requests: I) -> Self::Batch
    where
        I: IntoIterator<Item = (RequestId, rpc::Call)>,
    {
        (**self).send_batch(requests)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use api::Web3;
    use futures::Future;
    use super::{rpc, Error, RequestId, Transport};

    #[derive(Debug, Clone)]
    struct FakeTransport;

    impl Transport for FakeTransport {
        type Out = Box<Future<Item = rpc::Value, Error = Error> + Send + 'static>;

        fn prepare(&self, _method: &str, _params: Vec<rpc::Value>) -> (RequestId, rpc::Call) {
            unimplemented!()
        }

        fn send(&self, _id: RequestId, _request: rpc::Call) -> Self::Out {
            unimplemented!()
        }
    }

    #[test]
    fn should_allow_to_use_arc_as_transport() {
        let transport = Arc::new(FakeTransport);
        let transport2 = transport.clone();

        let _web3_1 = Web3::new(transport);
        let _web3_2 = Web3::new(transport2);
    }
}
