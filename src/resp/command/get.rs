use std::{collections::HashMap, io::{Error, ErrorKind, Result}, sync::MutexGuard};

use crate::resp::{array::RESPArrayElement, Nil, RESPDataType, SimpleString, StoredValue};

use super::StoringCommand;

#[derive(Debug)]
pub struct GetCommand {
    key: String,
}

impl StoringCommand for GetCommand {
    fn from_resp_array(arr: &Vec<RESPArrayElement>) -> Result<Self> {
        match arr.get(1) {
            Some(element) => Ok(Self { key: element.value.to_string() }),
            None => Err(Error::new(ErrorKind::InvalidInput, "Invalid echo command and arguments."))
        }
    }
    fn execute(&self, store: MutexGuard<HashMap<String, StoredValue>>) -> Result<String> {
        println!("GET");
        let key = &self.key;
        let get_result = store.get(key);
        match get_result {
            Some(val) => {
                if val.is_expired() {
                    return Nil::new().into_response_str();
                }
                SimpleString::new(&val.value).into_response_str()
            },
            _ => Nil::new().into_response_str(),
        }
    }
}
