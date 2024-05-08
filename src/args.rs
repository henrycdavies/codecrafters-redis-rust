use std::env;

pub struct Args {
    pub port: i16,
}

impl Args {
    pub fn from_env() -> Self {
        let args = env::args().collect();
        let port = Self::get_port(args);

        Self { port }
    }

    fn get_port(args: Vec<String>) -> i16 {
        let default: i16 = 6379;
        match args.iter().position(|arg| arg == "--port") {
            Some(idx) => args.get(idx + 1)
            .and_then(|v| v.parse().ok()).unwrap_or(default),
            None => default
        }
        
    }
}