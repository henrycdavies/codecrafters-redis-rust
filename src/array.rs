use crate::{command::RESPCommand, resp::RESPDataType, util::validate_lengths};

pub struct RESPArray;

impl RESPDataType for RESPArray {
    fn get_response<'a>(parts: &[&str]) -> &'a str {
        let arr = &parts[1..];
        let arr_size = arr.len() / 2;
        validate_lengths(parts[0], arr_size);

        let token = &parts[1][0..1];
        match token {
            "$" => RESPCommand::get_response(arr),
            _ => "UNIMPLEMENTED"
        }
    }
}