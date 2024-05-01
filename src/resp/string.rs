use std::io::Result;

use super::RESPDataType;

pub struct SimpleString {
    value: String,
}

impl SimpleString {
    pub fn new(string: &str) -> Self {
        let value = format!("+{}\r\n", string);
        SimpleString { value }
    }
}

impl<'a> RESPDataType<'a> for SimpleString {
    fn into_response_str(&self) -> Result<String> {
        Ok(self.value.to_string())
    }

    fn from_bytes(bytes: &'a [u8]) -> Result<Box<Self>> {
        unimplemented!()
    }

}