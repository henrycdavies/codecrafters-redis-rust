use std::{collections::HashMap, io::{Error, ErrorKind, Result}, sync::MutexGuard};

use crate::resp::{array::RESPArrayElement, RESPDataType, SimpleString, StoredValue};

use super::StoringCommand;



#[derive(Debug)]
pub struct SetCommand {
    key: String,
    value: String,
    px: Option<i64>
}

impl StoringCommand for SetCommand {
    fn from_resp_array(arr: &Vec<RESPArrayElement>) -> Result<Self> {
        let key = match arr.get(1) {
            Some(element) => element.value.to_string(),
            None => return Err(Error::new(ErrorKind::InvalidInput, "Invalid set command key."))
        };
        let value = match arr.get(2) {
            Some(element) => element.value.to_string(),
            None => return Err(Error::new(ErrorKind::InvalidInput, "Invalid set command value."))
        };
        let px = arr.get(3)
            .and_then(|v| {
                if v.value.to_uppercase() == "PX" {
                    return arr.get(4);
                }
                None
            })
            .and_then(|v| v.value.parse().ok() );
        Ok(Self { key, value, px })
    }
    fn execute(&self, mut store: MutexGuard<HashMap<String, StoredValue>>) -> Result<String> {
        println!("SET");
        let key = &self.key;
        let val = match self.px {
            Some(px) => {
                // let ttl: i64 = arg.value.parse().map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;
                StoredValue::new_with_ttl(&self.value, px)
            },
            None => {
                StoredValue::new(&self.value)
            },
        };
        store.insert(key.to_string(), val);
        SimpleString::new("OK").into_response_str()
    }
}
