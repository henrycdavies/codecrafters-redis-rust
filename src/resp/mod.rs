pub use array::Array;
pub use bulk_string::BulkString;
pub use command::Command;
pub use error::Error;
pub use nil::Nil;
pub use response::Response;
pub use string::SimpleString;
pub use value::StoredValue;

pub mod array;
pub mod bulk_string;
pub mod error;
pub mod nil;
pub mod response;
pub mod command;
pub mod string;
pub mod value;

pub trait RESPDataType<'a> {
    fn into_response_str(&'a self) -> String;
}

pub trait Bulk<'a>: RESPDataType<'a> {
    fn from_str_array(arr: &'a [&'a str]) -> Self;
}

pub const ARRAY_INDICATOR: u8 = b'*';
pub const COMMAND_INDICATOR: u8 = b'$';
pub const CRLF: &str = "\r\n";