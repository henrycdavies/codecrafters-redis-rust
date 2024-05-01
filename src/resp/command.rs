use std::{io::{Error, ErrorKind, Result}};
use crate::{resp::{Nil, RESPDataType}, KeyValueStore};
use super::{array::RESPArrayElement, BulkString, SimpleString};

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
        match s {
            "PING" => CommandName::PING,
            "ECHO" => CommandName::ECHO,
            "COMMAND" => CommandName::COMMAND,
            "GET" => CommandName::GET,
            "SET" => CommandName::SET,
            _ => unimplemented!()
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
    store: KeyValueStore,
}

impl CommandHandler {
    pub fn new(store: KeyValueStore) -> Self {
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
        let key = command.args[0].value;
        let store = self.store.lock().unwrap();
        let get_result = store.get(key);
        match get_result {
            Some(val) => SimpleString::new(val).into_response_str(),
            _ => Nil::new().into_response_str(),
        }
    }

    fn set(&self, command: Command) -> Result<String> {
        println!("SET");
        let key = command.args[0].value.to_string();
        let val = command.args[1].value.to_string();
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
}