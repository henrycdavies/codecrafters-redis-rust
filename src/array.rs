use crate::{command::RESPCommand, resp::RESPDataType, util::validate_lengths};

pub struct RESPArray<'a> {
    size: usize,
    parts: Vec<&'a str>
}

impl<'a> RESPDataType<'a> for RESPArray<'a> {
    fn from_str_array(parts: &'a [&'a str]) -> RESPArray<'a> {
        let arr = &parts[1..];
        let arr_size = arr.len() / 2;
        validate_lengths(parts[0], arr_size);
        RESPArray { size: arr_size, parts: arr.to_vec() }
    }
    fn get_response(&self) -> String {
        let token = &self.parts[0][0..1];

        match token {
            "$" => RESPCommand::from_str_array(&self.parts).get_response(),
            _ => "UNIMPLEMENTED".to_string()
        }
    }
}