use std::io::Result;

use super::RESPDataType;

pub struct Nil {
    value: String,
}

impl Nil {
    pub fn new() -> Self {
        let value = "$-1\r\n".to_string();
        Nil { value }
    }
}

impl<'a> RESPDataType<'a> for Nil {
    fn into_response_str(&self) -> Result<String> {
        Ok(self.value.to_string())
    }

    fn from_bytes(bytes: &'a [u8]) -> Result<Box<Self>> {
        unimplemented!()
    }

}