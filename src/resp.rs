pub trait RESPDataType {
    fn get_response<'a>(parts: &[&str]) -> &'a str;
}