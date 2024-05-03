pub mod echo;
pub mod get;
pub mod ping;
pub mod set;

use std::{collections::HashMap, io::{Error, ErrorKind, Result}, sync::MutexGuard};
use crate::{resp::RESPDataType, KeyValueStore};
use self::{echo::EchoCommand, get::GetCommand, ping::PingCommand, set::SetCommand};

use super::{array::RESPArrayElement, SimpleString, StoredValue};

trait BaseCommand: Sized {
    fn from_resp_array(arr: &Vec<RESPArrayElement>) -> Result<Self>;
    fn execute(&self) -> Result<String>;
}

trait StoringCommand: Sized {
    fn from_resp_array(arr: &Vec<RESPArrayElement>) -> Result<Self>;
    fn execute(&self, store: MutexGuard<HashMap<String, StoredValue>>) -> Result<String>;
}



#[derive(Debug)]
pub struct CommandCommand;

impl BaseCommand for CommandCommand {
    fn from_resp_array(_arr: &Vec<RESPArrayElement>) -> Result<Self> {
        Ok(Self { })
    }
    fn execute(&self) -> Result<String> {
        println!("COMMAND");
        SimpleString::new("COMMAND").into_response_str()
    }
}

#[derive(Debug)]
pub enum Command {
    Ping(PingCommand),
    Echo(EchoCommand),
    Command(CommandCommand),
    Get(GetCommand),
    Set(SetCommand),
}

impl Command {
    pub fn from_resp_array(arr: &Vec<RESPArrayElement>) -> Result<Self> {
        let name = arr[0].value;
        let uppercased = name.to_uppercase();
        match uppercased.as_str() {
            "PING" =>
                match PingCommand::from_resp_array(arr) {
                    Ok(cmd) => Ok(Command::Ping(cmd)),
                    _ => Err(Error::new(ErrorKind::InvalidInput, "Invalid input for command."))
                
            },
            "ECHO" =>
                match EchoCommand::from_resp_array(arr) {
                    Ok(cmd) => Ok(Command::Echo(cmd)),
                    _ => Err(Error::new(ErrorKind::InvalidInput, "Invalid input for command."))
                
            },
            "COMMAND" =>
                match CommandCommand::from_resp_array(arr) {
                    Ok(cmd) => Ok(Command::Command(cmd)),
                    _ => Err(Error::new(ErrorKind::InvalidInput, "Invalid input for command."))
                
            },
            "GET" =>
                match GetCommand::from_resp_array(arr) {
                    Ok(cmd) => Ok(Command::Get(cmd)),
                    _ => Err(Error::new(ErrorKind::InvalidInput, "Invalid input for command."))
                
            },
            "SET" =>
                match SetCommand::from_resp_array(arr) {
                    Ok(cmd) => Ok(Command::Set(cmd)),
                    _ => Err(Error::new(ErrorKind::InvalidInput, "Invalid input for command."))
                
            },
            _ => {
                println!("Error encountered parsing command from string {}", uppercased);
                unimplemented!()
            }
        }
    }
}

pub struct CommandHandler {
    store: KeyValueStore<StoredValue>,
}

impl CommandHandler {
    pub fn new(store: KeyValueStore<StoredValue>) -> Self {
        CommandHandler { store }
    }

    pub fn execute(&self, command: Command) -> Result<String> {
        let _store = self.store.lock().unwrap();
        match command {
            Command::Echo(cmd) => cmd.execute(),
            Command::Command(cmd) => cmd.execute(),
            Command::Ping(cmd) => cmd.execute(),
            Command::Set(cmd) => cmd.execute(_store),
            Command::Get(cmd) => cmd.execute(_store),
        }
    }
}