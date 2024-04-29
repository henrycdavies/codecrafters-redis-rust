use crate::{resp::RESPDataType, util::validate_lengths};

pub struct RESPCommand<'a> {
    command: String,
    args: Vec<&'a str>
}

impl<'a> RESPCommand<'a> {
    fn ping(args: &[&str]) -> String {
        "+PONG\r\n".to_string()
    }

    fn echo(args: &[&str]) -> String {
        let to_echo = args[1];
        let len = to_echo.len();
        format!("${}\r\n{}\r\n", len, to_echo)
    }

    fn command() -> String {
        "UNIMPLEMENTED".to_string()
    }
}

impl<'a> RESPDataType<'a> for RESPCommand<'a> {
    fn from_str_array(parts: &'a [&'a str]) -> RESPCommand<'a> {
        validate_lengths(parts[0], parts[1].len());
        let command = parts[1].trim().to_uppercase();
        let args = parts[2..].to_vec();
        RESPCommand { command, args }
    }
    fn get_response(&self) -> String {
        match self.command.as_str() {
            "PING" => Self::ping(&self.args),
            "ECHO" => Self::echo(&self.args),
            "COMMAND" => Self::command(),
            _ => "UNIMPLEMENTED".to_string()
        }
    }
}
