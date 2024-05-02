use std::{env::Args, io::{Error, ErrorKind, Result}};
use crate::{resp::{Nil, RESPDataType}, KeyValueStore};
use super::{array::RESPArrayElement, BulkString, SimpleString, StoredValue};

#[derive(Debug)]
pub enum CommandName {
    PING,
    ECHO,
    COMMAND,
    GET,
    SET,
}

impl CommandName {
    pub fn from(s: &str) -> Self {
        let uppercased = s.to_uppercase();
        match uppercased.as_str() {
            "PING" => CommandName::PING,
            "ECHO" => CommandName::ECHO,
            "COMMAND" => CommandName::COMMAND,
            "GET" => CommandName::GET,
            "SET" => CommandName::SET,
            _ => {
                println!("Error encountered parsing command from string {}", s);
                unimplemented!()
            }
        }
    }
}

pub struct Command<'a> {
    name: CommandName,
    args: Vec<RESPArrayElement<'a>>
}

impl<'a> Command<'a> {
    pub fn from_resp_array(arr: &Vec<RESPArrayElement<'a>>) -> Command<'a> {
        // let command = arr[0].value.to_string();
        let command = CommandName::from(&arr[0].value);
        println!("COMMAND: {:?}", command);
        let args: Vec<RESPArrayElement> = arr[1..].to_vec();
        Command { name: command, args }
    }
}

pub struct CommandHandler {
    store: KeyValueStore<StoredValue>,
}

impl CommandHandler {
    pub fn new(store: KeyValueStore<StoredValue>) -> Self {
        CommandHandler { store }
    }

    fn ping(&self, command: Command) -> Result<String> {
        println!("PING");
        SimpleString::new("PONG").into_response_str()
    }

    fn echo(&self, command: Command) -> Result<String> {
        println!("ECHO");
        let val = command.args[0].value;
        BulkString::new(val).into_response_str()
    }

    fn command(&self, command: Command) -> Result<String> {
        println!("COMMAND");
        SimpleString::new("COMMAND").into_response_str()
    }

    fn get(&self, command: Command) -> Result<String> {
        println!("GET");
        let key = self.extract_key_or_err(&command, 0)?;
        let store = self.store.lock().unwrap();
        let get_result = store.get(&key);
        match get_result {
            Some(val) => {
                if val.is_expired() {
                    return Nil::new().into_response_str();
                }
                SimpleString::new(&val.value).into_response_str()
            },
            _ => Nil::new().into_response_str(),
        }
    }

    fn set(&self, command: Command) -> Result<String> {
        println!("SET");
        let key = self.extract_key_or_err(&command, 0)?;
        let val = self.extract_key_or_err(&command, 1)?;
        let val = match command.args.get(2) {
            Some(arg) => {
                let ttl: i64 = arg.value.parse().map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;
                StoredValue::new_with_ttl(&val, ttl)
            },
            None => {
                StoredValue::new(command.args[1].value)
            },
        };
        self.store.lock().unwrap().insert(key, val);
        SimpleString::new("OK").into_response_str()
    }

    pub fn execute(&self, command: Command) -> Result<String> {
        match command.name {
            CommandName::PING => self.ping(command),
            CommandName::ECHO => self.echo(command),
            CommandName::COMMAND => self.command(command),
            CommandName::SET => self.set(command),
            CommandName::GET => self.get(command),
            _ => Err(Error::new(ErrorKind::InvalidInput, "Invalid command")),
        }
    }

    fn extract_key_or_err(&self, command: &Command, arg_index: usize) -> Result<String> {
        if let Some(v) = command.args.get(arg_index) {
            Ok(v.value.to_string())
        } else {
            let message = format!("Missing arg number {} for {:?} command.", arg_index, command.name);
            return Err(Error::new(ErrorKind::InvalidInput, message))
        }
    }

    fn extract_key_or_default(&self, command: Command, arg_index: usize, default: String) -> String {
        if let Some(v) = command.args.get(arg_index) {
            v.value.to_string()
        } else {
            default.to_string()
        }
    }
}