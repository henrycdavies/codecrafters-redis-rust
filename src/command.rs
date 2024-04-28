use crate::{resp::RESPDataType, util::validate_lengths};

pub struct RESPCommand;

impl RESPCommand {
    fn ping<'a>(args: &[&str]) -> &'a str {
        "+PONG\r\n"
    }

    fn command<'a>() -> &'a str {
        "UNIMPLEMENTED"
    }
}

impl RESPDataType for RESPCommand {
    fn get_response<'a>(parts: &[&str]) -> &'a str {
        validate_lengths(parts[0], parts[1].len());
        let command = parts[1].trim().to_uppercase();
        match command.as_str() {
            "PING" => Self::ping(&parts[2..]),
            "COMMAND" => Self::command(),
            _ => "UNIMPLEMENTED"
        }
    }
}
