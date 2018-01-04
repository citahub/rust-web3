//! `Cita` namespace
#![allow(dead_code, unused_imports)]
use api::Namespace;
use helpers::{self, CallResult};
use types::{Address, BlockId, BlockNumber, Bytes, H256, U256, Work};
use Transport;
use cita_types::*;

/// Cita
#[derive(Debug, Clone)]
pub struct Cita<T> {
    transport: T,
}

impl<T: Transport> Namespace<T> for Cita<T> {
    fn new(transport: T) -> Self
    where
        Self: Sized,
    {
        Cita { transport }
    }

    fn transport(&self) -> &T {
        &self.transport
    }
}

impl<T: Transport> Cita<T> {
    /// Get list of available accounts.
    fn accounts(&self) -> CallResult<Vec<Address>, T::Out> {
        CallResult::new(self.transport.execute("eth_accounts", vec![]))
    }

    /// Get current block number
    pub fn block_number(&self) -> CallResult<U256, T::Out> {
        CallResult::new(self.transport.execute("cita_blockNumber", vec![]))
    }

    /// Call a constant method of contract without changing the state of the blockchain.
    pub fn call(&self, req: CallRequest, block: Option<BlockNumber>) -> CallResult<Bytes, T::Out> {
        let req = helpers::serialize(&req);
        let block = helpers::serialize(&block.unwrap_or(BlockNumber::Latest));

        CallResult::new(self.transport.execute("eth_call", vec![req, block]))
    }

    /// Get block details with transaction hashes.
    pub fn block(&self, block: BlockId) -> CallResult<Block, T::Out> {
        let include_txs = helpers::serialize(&false);

        let result = match block {
            BlockId::Hash(hash) => {
                let hash = helpers::serialize(&hash);
                self.transport
                    .execute("cita_getBlockByHash", vec![hash, include_txs])
            }
            BlockId::Number(num) => {
                let num = helpers::serialize(&num);
                self.transport
                    .execute("cita_getBlockByNumber", vec![num, include_txs])
            }
        };

        CallResult::new(result)
    }

    /// Get block details with full transaction objects.
    pub fn block_with_txs(&self, block: BlockId) -> CallResult<Block, T::Out> {
        let include_txs = helpers::serialize(&true);

        let result = match block {
            BlockId::Hash(hash) => {
                let hash = helpers::serialize(&hash);
                self.transport
                    .execute("cita_getBlockByHash", vec![hash, include_txs])
            }
            BlockId::Number(num) => {
                let num = helpers::serialize(&num);
                self.transport
                    .execute("cita_getBlockByNumber", vec![num, include_txs])
            }
        };

        CallResult::new(result)
    }

    /// Get number of transactions in block
    fn block_transaction_count(&self, block: BlockId) -> CallResult<Option<U256>, T::Out> {
        let result = match block {
            BlockId::Hash(hash) => {
                let hash = helpers::serialize(&hash);
                self.transport
                    .execute("eth_getBlockTransactionCountByHash", vec![hash])
            }
            BlockId::Number(num) => {
                let num = helpers::serialize(&num);
                self.transport
                    .execute("eth_getBlockTransactionCountByNumber", vec![num])
            }
        };

        CallResult::new(result)
    }

    /// Get code under given address
    pub fn code(&self, address: Address, block: Option<BlockNumber>) -> CallResult<Bytes, T::Out> {
        let address = helpers::serialize(&address);
        let block = helpers::serialize(&block.unwrap_or(BlockNumber::Latest));

        CallResult::new(self.transport.execute("eth_getCode", vec![address, block]))
    }

    /// Get storage entry
    fn storage(&self, address: Address, idx: U256, block: Option<BlockNumber>) -> CallResult<H256, T::Out> {
        let address = helpers::serialize(&address);
        let idx = helpers::serialize(&idx);
        let block = helpers::serialize(&block.unwrap_or(BlockNumber::Latest));

        CallResult::new(
            self.transport
                .execute("eth_getStorageAt", vec![address, idx, block]),
        )
    }

    /// Get nonce
    pub fn transaction_count(&self, address: Address, block: Option<BlockNumber>) -> CallResult<U256, T::Out> {
        let address = helpers::serialize(&address);
        let block = helpers::serialize(&block.unwrap_or(BlockNumber::Latest));

        CallResult::new(
            self.transport
                .execute("eth_getTransactionCount", vec![address, block]),
        )
    }

    /// Get transaction
    pub fn transaction(&self, hash: H256) -> CallResult<Option<RpcTransaction>, T::Out> {
        let hash = helpers::serialize(&hash);
        CallResult::new(self.transport.execute("cita_getTransaction", vec![hash]))
    }

    /// Get transaction receipt
    pub fn transaction_receipt(&self, hash: H256) -> CallResult<Option<Receipt>, T::Out> {
        let hash = helpers::serialize(&hash);

        CallResult::new(
            self.transport
                .execute("eth_getTransactionReceipt", vec![hash]),
        )
    }

    /// Get work package
    fn work(&self) -> CallResult<Work, T::Out> {
        CallResult::new(self.transport.execute("eth_getWork", vec![]))
    }

    /// Start new block filter
    pub fn new_block_filter(&self) -> CallResult<U256, T::Out> {
        CallResult::new(self.transport.execute("eth_newBlockFilter", vec![]))
    }

    /// Start new pending transaction filter
    fn new_pending_transaction_filter(&self) -> CallResult<U256, T::Out> {
        CallResult::new(
            self.transport
                .execute("eth_newPendingTransactionFilter", vec![]),
        )
    }

    /// Start new pending transaction filter
    fn protocol_version(&self) -> CallResult<String, T::Out> {
        CallResult::new(self.transport.execute("eth_protocolVersion", vec![]))
    }

    /// Sends a rlp-encoded signed transaction
    fn send_raw_transaction(&self, rlp: Bytes) -> CallResult<H256, T::Out> {
        let rlp = helpers::serialize(&rlp);
        CallResult::new(self.transport.execute("eth_sendRawTransaction", vec![rlp]))
    }

    /// Sends a transaction transaction
    pub fn send_transaction(&self, tx: String) -> CallResult<TxResponse, T::Out> {
        let tx = helpers::serialize(&tx);
        CallResult::new(self.transport.execute("cita_sendTransaction", vec![tx]))
    }

    // TODO [ToDr] Proper type?
    /// Get syncing status
    fn syncing(&self) -> CallResult<bool, T::Out> {
        CallResult::new(self.transport.execute("eth_syncing", vec![]))
    }
}
//
//#[cfg(test)]
//mod tests {
//    use futures::Future;
//
//    use api::Namespace;
//    use types::{Block, BlockId, BlockNumber, Bytes, CallRequest, H256, Transaction, TransactionId, TransactionReceipt,
//                TransactionRequest, Work};
//    use rpc::Value;
//
//    use super::Cita;
//
//    // taken from RPC docs.
//    const EXAMPLE_BLOCK: &'static str = r#"{
//    "number": "0x1b4",
//    "hash": "0x0e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d1527331",
//    "parentHash": "0x9646252be9520f6e71339a8df9c55e4d7619deeb018d2a3f2d21fc165dde5eb5",
//    "sealFields": [
//      "0xe04d296d2460cfb8472af2c5fd05b5a214109c25688d3704aed5484f9a7792f2",
//      "0x0000000000000042"
//    ],
//    "sha3Uncles": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
//    "logsBloom":  "0x0e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec643\
// 41771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3d5\
// 145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec6\
// 4341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3\
// d5145a99d05921026d15273310e670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d15273310e670ec\
// 64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d1527331",
//    "transactionsRoot": "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
//    "receiptsRoot": "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
//    "stateRoot": "0xd5855eb08b3387c0af375e9cdb6acfc05eb8f519e419b874b6ff2ffda7ed1dff",
//    "miner": "0x4e65fda2159562a496f9f3522f89122a3088497a",
//    "difficulty": "0x27f07",
//    "totalDifficulty": "0x27f07",
//    "extraData": "0x0000000000000000000000000000000000000000000000000000000000000000",
//    "size": "0x27f07",
//    "gasLimit": "0x9f759",
//    "minGasPrice": "0x9f759",
//    "gasUsed": "0x9f759",
//    "timestamp": "0x54e34e8e",
//    "transactions": [],
//    "uncles": []
//  }"#;
//
//    // taken from RPC docs.
//    const EXAMPLE_TX: &'static str = r#"{
//    "hash": "0xc6ef2fc5426d6ad6fd9e2a26abeab0aa2411b7ab17f30a99d3cb96aed1d1055b",
//    "nonce": "0x0",
//    "blockHash": "0xbeab0aa2411b7ab17f30a99d3cb9c6ef2fc5426d6ad6fd9e2a26a6aed1d1055b",
//    "blockNumber": "0x15df",
//    "transactionIndex": "0x1",
//    "from": "0x407d73d8a49eeb85d32cf465507dd71d507100c1",
//    "to":   "0x85dd43d8a49eeb85d32cf465507dd71d507100c1",
//    "value": "0x7f110",
//    "gas": "0x7f110",
//    "gasPrice": "0x09184e72a000",
//    "input": "0x603880600c6000396000f300603880600c6000396000f3603880600c6000396000f360"
//  }"#;
//
//    // taken from RPC docs.
//    const EXAMPLE_RECEIPT: &'static str = r#"{
//    "hash": "0xb903239f8543d04b5dc1ba6579132b143087c68db1b2168786408fcbce568238",
//    "index": "0x1",
//    "transactionHash": "0xb903239f8543d04b5dc1ba6579132b143087c68db1b2168786408fcbce568238",
//    "transactionIndex": "0x1",
//    "blockNumber": "0xb",
//    "blockHash": "0xc6ef2fc5426d6ad6fd9e2a26abeab0aa2411b7ab17f30a99d3cb96aed1d1055b",
//    "cumulativeGasUsed": "0x33bc",
//    "gasUsed": "0x4dc",
//    "contractAddress": "0xb60e8dd61c5d32be8058bb8eb970870f07233155",
//    "logs": []
//  }"#;
//
//
//    rpc_test! (
//    Eth:block_number => "eth_blockNumber";
//    Value::String("0x123".into()) => 0x123
//  );
//
//    rpc_test! (
//    Eth:call, CallRequest {
//      from: None, to: 0x123.into(),
//      gas: None, gas_price: None,
//      value: Some(0x1.into()), data: None,
//    }, None
//    =>
//    "eth_call", vec![r#"{"to":"0x0000000000000000000000000000000000000123","value":"0x1"}"#, \
// r#""latest""#];
//    Value::String("0x010203".into()) => Bytes(vec![1, 2, 3])
//  );
//
//
//    rpc_test! (
//    Eth:block:block_by_hash, BlockId::Hash(0x123.into())
//    =>
//    "cita_getBlockByHash", vec![r#""0x0000000000000000000000000000000000000000000000000000000\
// 000000123""#, r#"false"#];
//    ::serde_json::from_str(EXAMPLE_BLOCK).unwrap()
//    => ::serde_json::from_str::<Block<H256>>(EXAMPLE_BLOCK).unwrap()
//  );
//
//    rpc_test! (
//    Eth:block, BlockNumber::Pending
//    =>
//    "cita_getBlockByNumber", vec![r#""pending""#, r#"false"#];
//    ::serde_json::from_str(EXAMPLE_BLOCK).unwrap()
//    => ::serde_json::from_str::<Block<H256>>(EXAMPLE_BLOCK).unwrap()
//  );
//
//
//    rpc_test! (
//    Eth:code, 0x123, Some(BlockNumber::Pending)
//    =>
//    "eth_getCode", vec![r#""0x0000000000000000000000000000000000000123""#, r#""pending""#];
//    Value::String("0x0123".into()) => Bytes(vec![0x1, 0x23])
//  );
//
//    rpc_test! (
//    Eth:transaction_count, 0x123, None
//    =>
//    "eth_getTransactionCount", vec![r#""0x0000000000000000000000000000000000000123""#, r#""latest""#];
//    Value::String("0x123".into()) => 0x123
//  );
//
//    rpc_test! (
//    Eth:transaction:tx_by_hash, TransactionId::Hash(0x123.into())
//    =>
//    "cita_getTransaction", vec![r#""0x0000000000000000000000000000000000000000000000000000000000000123""#];
//    ::serde_json::from_str(EXAMPLE_TX).unwrap()
//    => Some(::serde_json::from_str::<Transaction>(EXAMPLE_TX).unwrap())
//  );
//
//
//    rpc_test! (
//    Eth:transaction_receipt, 0x123
//    =>
//    "eth_getTransactionReceipt", vec![r#""0x0000000000000000000000000000000000000000000000000000000000000123""#];
//    ::serde_json::from_str(EXAMPLE_RECEIPT).unwrap()
//    => Some(::serde_json::from_str::<TransactionReceipt>(EXAMPLE_RECEIPT).unwrap())
//  );
//
//    rpc_test! (
//    Eth:new_block_filter => "eth_newBlockFilter";
//    Value::String("0x123".into()) => 0x123
//  );
//    rpc_test! (
//    Eth:new_pending_transaction_filter => "eth_newPendingTransactionFilter";
//    Value::String("0x123".into()) => 0x123
//  );
//
//    rpc_test! (
//    Eth:send_raw_transaction, Bytes(vec![1, 2, 3, 4])
//    =>
//    "eth_sendRawTransaction", vec![r#""0x01020304""#];
//    Value::String("0x0000000000000000000000000000000000000000000000000000000000000123".into()) => 0x123
//  );
//
//    rpc_test! (
//    Eth:send_transaction, TransactionRequest {
//      from: 0x123.into(), to: Some(0x123.into()),
//      gas: None, gas_price: Some(0x1.into()),
//      value: Some(0x1.into()), data: None,
//      nonce: None, condition: None,
//    }
//    =>
//    "cita_sendTransaction", vec![r#"{"from":"0x0000000000000000000000000000000000000123",\
// "gasPrice":"0x1","to":"0x0000000000000000000000000000000000000123","value":"0x1"}"#];
//    Value::String("0x0000000000000000000000000000000000000000000000000000000000000123".into()) => 0x123
//  );
//}
