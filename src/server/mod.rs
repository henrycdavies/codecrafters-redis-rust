pub mod replication;

pub use replication::ReplicationRole;

use crate::{args::Args, resp::{Error, RESPDataType}};

use self::replication::ReplicationInfo;

#[derive(Debug)]
pub struct Info {
    pub replication: ReplicationInfo
}

impl Info {
    pub fn new(args: Args) -> Self {
        let replication_info = ReplicationInfo::new(args);
        Info { replication: replication_info }
    }

    pub fn get_section(&self, requested_section: String) -> String {
        match requested_section.as_str() {
            "replication" => self.replication.to_string(),
            _ => Error::new(stringify!("Invalid INFO section: {}", requested_section)).into_response_str(),
        }
    }
}