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
    fn into_response_str(&self) -> String {
        self.value.to_string()
    }
}