use super::CHUNK_SIZE;
use std::fs::File;
use std::io::{self, BufReader, Read, Result as IoResult};

pub fn read_input(infile: &str) -> IoResult<Vec<u8>> {
    let mut reader: Box<dyn Read> = if !infile.is_empty() {
        Box::new(BufReader::new(File::open(infile)?))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };
    let mut buffer = [0; CHUNK_SIZE];
    let num_read = reader.read(&mut buffer)?;
    Ok(Vec::from(&buffer[..num_read]))
}