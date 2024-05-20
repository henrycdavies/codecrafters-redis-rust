use crate::args::Args;

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
    pub master_replid: String,
    pub role: ReplicationRole,
    pub master_repl_offset: i64,
}

impl ReplicationInfo {
    pub fn new(args: Args) -> Self {
        let role = args.role;
        ReplicationInfo { role, master_replid: "8371b4fb1155b71f4a04d3e1bc3e18c4a990aeeb".to_string(), master_repl_offset: 0 }
    }

    pub fn to_string(&self) -> String {
        format!(
r#"master_replid:{}
{}
master_repl_offset:{}"#,
        self.master_replid,
        self.role.to_string(),
        self.master_repl_offset
    )
    }
}