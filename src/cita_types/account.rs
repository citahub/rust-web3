use serde_json::from_reader;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use cita_crypto::{PrivKey, PubKey};
use util::Address;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub secret: PrivKey,
    pub public: PubKey,
    pub address: Address,
}

impl Account {
    pub fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Account>, Box<Error>> {
        // Open the file in read-only mode.
        let file = File::open(path)?;
        // Read the JSON contents of the file as an instance of `User`.
        let u = from_reader(file)?;
        println!("{:?}", u);
        Ok(u)
    }
}
