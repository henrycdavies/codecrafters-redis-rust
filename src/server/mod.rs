use crate::{args::Args, resp::{Error, RESPDataType}};

#[derive(Debug)]
pub enum ReplicationRole {
    Master,
    Slave,
}

impl ReplicationRole {
    pub fn from_str(role: &str) -> Self {
        match role {
            "role:slave" => ReplicationRole::Slave,
            _ => ReplicationRole::Master,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ReplicationRole::Master => "role:master".to_string(),
            ReplicationRole::Slave => "role:slave".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct ReplicationInfo {
    pub role: ReplicationRole,
}

impl ReplicationInfo {
    pub fn new(args: Args) -> Self {
        let role = args.role;
        ReplicationInfo { role }
    }

    pub fn to_string(&self) -> String {
        format!(
r#"{}"#,
        self.role.to_string())
    }
}

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