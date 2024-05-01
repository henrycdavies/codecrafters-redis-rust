use std::io::{Error, ErrorKind, Result};

use crate::util::validate_lengths;

use super::{RESPDataType, Command, ARRAY_INDICATOR, CRLF};

#[derive(Clone)]
pub struct RESPArrayElement<'a> {
    pub indicator: u8,
    pub size: usize,
    pub value: &'a str,
}

impl<'a> RESPArrayElement<'a> {
    pub fn from_str_vec(parts: Vec<&'a str>) -> Result<Vec<RESPArrayElement<'a>>> {
        let mut elements = Vec::new();
        if elements.len() % 2 != 0 {
            return Err(Error::new(ErrorKind::InvalidInput, "Invalid number of elements in array"));
        }
        for i in 0..parts.len() - 1 {
            if i % 2 == 0 {
                let bytes = parts[i].as_bytes();
                let indicator = bytes[0];
                let size_str = std::str::from_utf8(&bytes[1..]).unwrap_or_default();
                let expected_size = size_str.parse::<usize>().unwrap_or_default();
                let actual_size = parts[i + 1].len();
                if expected_size != actual_size {
                    let message = format!("Invalid message: The specified size in element {} does not match the actual size of element {}", size_str, parts[i + 1]);
                    return Err(Error::new(ErrorKind::InvalidInput, message));
                }
                elements.push(RESPArrayElement { indicator, size: expected_size, value: parts[i + 1] });
            }
        }
        Ok(elements)
    }
}

pub struct Array<'a> {
    pub size: usize,
    pub parts: Vec<RESPArrayElement<'a>>
}

impl<'a> Array<'a> {
    fn from_resp_array(arr: Vec<RESPArrayElement<'a>>) -> Self {
        Array { size: arr.len(), parts: arr }
    }
}


impl<'a> RESPDataType<'a> for Array<'a> {
    fn from_bytes(bytes: &'a [u8]) -> Result<Box<Self>> {
        let first_byte = &bytes[0..1][0];
        if *first_byte != ARRAY_INDICATOR {
            return Err(Error::new(std::io::ErrorKind::InvalidInput, "Invalid input"));
        }
        match std::str::from_utf8(&bytes) {
            Ok(v) => {
                let parts: Vec<&str> = v.split(CRLF).collect();
                let elements = parts[1..].to_vec();
                let arr = RESPArrayElement::from_str_vec(elements).unwrap_or_default();
                validate_lengths(parts[0], arr.len());
                Ok(Box::new(Array::from_resp_array(arr)))
            },
            Err(e) => 
                Err(Error::new(std::io::ErrorKind::InvalidData, e.to_string()))
        }        
    }

    fn into_response_str(&self) -> Result<String> {
        unimplemented!()
    }
}