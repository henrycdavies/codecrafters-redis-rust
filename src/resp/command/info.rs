use std::{io::Result, sync::Arc};

use crate::{resp::{BulkString, RESPDataType}, server::Info};

use super::AdministrativeCommand;

#[derive(Debug)]
pub struct InfoCommand {
    pub section: Option<String>,
}

impl AdministrativeCommand for InfoCommand {
    fn from_resp_array(arr: &Vec<crate::resp::array::RESPArrayElement>) -> Result<Self> {
        match arr.get(1) {
            Some(element) => Ok(Self { section: Some(element.value.to_string()) }),
            None => Ok(Self { section: None })
        }
    }

    fn execute(&self, server_info: Arc<Info>) -> Result<String> {
        match &self.section {
            Some(sec) => {
                let info_string = server_info.as_ref().get_section(sec.to_string());
                Ok(BulkString::new(&info_string).into_response_str())
            },
            _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid INFO command and arguments.")),
        }
    }
}