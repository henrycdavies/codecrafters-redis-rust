pub mod echo;
pub mod get;
pub mod info;
pub mod ping;
pub mod set;

use std::{collections::HashMap, io::{Error, ErrorKind, Result}, sync::{Arc, MutexGuard}};
use crate::{resp::RESPDataType, server::Info, KeyValueStore};
use self::{echo::EchoCommand, get::GetCommand, info::InfoCommand, ping::PingCommand, set::SetCommand};

use super::{array::RESPArrayElement, error, SimpleString, StoredValue};

trait BaseCommand: Sized {
    fn from_resp_array(arr: &Vec<RESPArrayElement>) -> Result<Self>;
    fn execute(&self) -> Result<String>;
}

trait StoringCommand: Sized {
    fn from_resp_array(arr: &Vec<RESPArrayElement>) -> Result<Self>;
    fn execute(&self, store: MutexGuard<HashMap<String, StoredValue>>) -> Result<String>;
}

trait AdministrativeCommand: Sized {
    fn from_resp_array(arr: &Vec<RESPArrayElement>) -> Result<Self>;
    fn execute(&self, server_info: Arc<Info>) -> Result<String>;
}

#[derive(Debug)]
pub struct CommandCommand;

impl BaseCommand for CommandCommand {
    fn from_resp_array(_arr: &Vec<RESPArrayElement>) -> Result<Self> {
        Ok(Self { })
    }
    fn execute(&self) -> Result<String> {
        Ok(SimpleString::new("OK").into_response_str())
    }
}

#[derive(Debug)]
pub enum Command {
    Command(CommandCommand),
    Echo(EchoCommand),
    Get(GetCommand),
    Info(InfoCommand),
    Ping(PingCommand),
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
            "INFO" =>
                match InfoCommand::from_resp_array(arr) {
                    Ok(cmd) => Ok(Command::Info(cmd)),
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
    server_info: Arc<Info>,
}

impl CommandHandler {
    pub fn new(store: KeyValueStore<StoredValue>, server_info: Arc<Info>) -> Self {
        CommandHandler { store, server_info }
    }

    pub fn execute(&self, command: Command) -> String {
        let _store = self.store.lock().unwrap();
        let _server_info = self.server_info.clone();
        let exec_result = match command {
            Command::Command(cmd) => cmd.execute(),
            Command::Echo(cmd) => cmd.execute(),
            Command::Get(cmd) => cmd.execute(_store),
            Command::Info(cmd) => cmd.execute(_server_info),
            Command::Ping(cmd) => cmd.execute(),
            Command::Set(cmd) => cmd.execute(_store),
        };
        match exec_result {
            Ok(response_str) => response_str,
            _ => error::Error::new("ERR unexpected error.").into_response_str(),
        }
    }
}