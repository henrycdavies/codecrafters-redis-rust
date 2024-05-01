use std::io::Result;

use super::RESPDataType;

pub struct BulkString<'a> {
    size: usize,
    value: &'a str,
}


impl<'a> BulkString<'a> {
    pub fn new(value: &'a str) -> Self {
        let size = value.len();

        BulkString { size, value }
    }
}

impl<'a> RESPDataType<'a> for BulkString<'a> {
    fn into_response_str(&'a self) -> Result<String> {
        let msg = format!("${}\r\n{}\r\n", self.size, self.value);
        Ok(msg)
    }

    fn from_bytes(bytes: &'a [u8]) -> Result<Box<Self>> {
        unimplemented!()
    }
}