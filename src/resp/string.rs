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
    fn into_response_str(&self) -> String {
        self.value.to_string()
    }
}