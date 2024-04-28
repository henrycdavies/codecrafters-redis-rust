use std::io::{Error, ErrorKind, Result};

pub fn validate_lengths(description: &str, actual: usize) -> Result<()> {
    let prediction = &description[1..].parse::<usize>().unwrap();
    if *prediction != actual {
        println!("Lengths do not match: {}, {}", description, actual);
        Err(Error::new(ErrorKind::InvalidData, "Lengths do not match"))
    } else {
        Ok(())
    }
}