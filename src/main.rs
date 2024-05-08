pub mod args;
pub mod resp;
pub mod pool;
pub mod store;
pub mod util;

use std::{io::{Error, ErrorKind, Read, Result, Write}, net::{TcpListener, TcpStream}, sync::Arc};

use args::Args;
use resp::{command::CommandHandler, Array, Command, RESPDataType, StoredValue, COMMAND_INDICATOR};
use pool::ThreadPool;
use store::{create_key_value_store, KeyValueStore};

fn main() {
    let args = Args::from_env();
    let addr = format!("127.0.0.1:{}", args.port);
    println!("Listening on {}", addr);
    let listener = TcpListener::bind(addr).unwrap();
    let pool = ThreadPool::new(4);
    let store: KeyValueStore<StoredValue> = create_key_value_store();
    for stream in listener.incoming() {
        if let Ok(_stream) = stream {
            let _store = Arc::clone(&store);
            pool.execute(|| {
                handle_stream(_stream, _store);
            });
        }
    }
}

fn handle_stream(mut stream: TcpStream, store: KeyValueStore<StoredValue>) -> Result<()> {
    let mut buf = [0; 512];
    let command_handler = CommandHandler::new(store);
    while let Ok(_) = stream.read(&mut buf) {
        let resp_array = match Array::from_bytes(&buf) {
            Ok(x) => x,
            Err(_) => return Err(Error::new(ErrorKind::InvalidInput, "Invalid message"))
        };
        if resp_array.parts[0].indicator != COMMAND_INDICATOR {
            return Err(Error::new(ErrorKind::InvalidInput, "Invalid input"));
        }
        if let Ok(command) = Command::from_resp_array(&resp_array.parts) {
            if let Ok(response) = command_handler.execute(command) {
                match stream.write_all(response.as_bytes()) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
            }
        };
    }
    Ok(())
}


