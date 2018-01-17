/// Call request
use types::{Address, Bytes};

/// eth_call param
#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CallRequest {
    /// From
    pub from: Option<Address>,
    /// To
    pub to: Address,
    /// Data
    pub data: Option<Bytes>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use serde_json::Error;

    #[test]
    fn call_deserialization() {
        let s = "{\"from\":\"0xd46e8dd67c5d32be8058bb8eb970870f07244567\",\
                 \"to\":\"0xb60e8dd61c5d32be8058bb8eb970870f07233155\",\
                 \"data\":\"0xd46e8dd67c5d32be8d46e8dd67c5d32be8058bb8eb970870f072445675058bb8eb970870f072445675\"}";
        let call: Result<CallRequest, Error> = serde_json::from_str(s);
        assert!(call.is_ok());
    }
}
