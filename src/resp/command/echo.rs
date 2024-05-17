use std::io::{Error, ErrorKind, Result};

use crate::resp::{array::RESPArrayElement, BulkString, RESPDataType};

use super::BaseCommand;

#[derive(Debug)]
pub struct EchoCommand {
    value: String,
}

impl BaseCommand for EchoCommand {
    fn from_resp_array(arr: &Vec<RESPArrayElement>) -> Result<Self> {
        match arr.get(1) {
            Some(element) => Ok(Self { value: element.value.to_string() }),
            None => Err(Error::new(ErrorKind::InvalidInput, "Invalid echo command and arguments."))
        }
    }
    fn execute(&self) -> Result<String> {
        Ok(BulkString::new(&self.value).into_response_str())
    }
}