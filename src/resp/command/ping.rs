use std::io::Result;

use crate::resp::{array::RESPArrayElement, RESPDataType, SimpleString};

use super::BaseCommand;

#[derive(Debug)]
pub struct PingCommand;

impl BaseCommand for PingCommand {
    fn from_resp_array(arr: &Vec<RESPArrayElement>) -> Result<Self> {
        let _ = arr;
        Ok(Self { })
    }
    fn execute(&self) -> Result<String> {
        Ok(SimpleString::new("PONG").into_response_str())
    }
}