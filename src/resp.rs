pub trait RESPDataType<'a> {
    fn from_str_array(parts: &'a [&'a str]) -> Self;
    fn get_response(&self) -> String;
}