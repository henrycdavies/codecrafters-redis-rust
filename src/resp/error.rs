use super::RESPDataType;

pub struct Error {
    value: String,
}

impl Error {
    pub fn new(string: &str) -> Self {
        let value = format!("-{}\r\n", string);
        Error { value }
    }
}

impl<'a> RESPDataType<'a> for Error {
    fn into_response_str(&self) -> String {
        self.value.to_string()
    }
}