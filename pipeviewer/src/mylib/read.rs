use super::CHUNK_SIZE;
use std::fs::File;
use std::io::{self, BufReader, Read, Result as IoResult};
use std::sync::{Arc, Mutex};

// pub fn read_input(infile: &str) -> IoResult<Vec<u8>> {
//     let mut reader: Box<dyn Read> = if !infile.is_empty() {
//         Box::new(BufReader::new(File::open(infile)?))
//     } else {
//         Box::new(BufReader::new(io::stdin()))
//     };
//     let mut buffer = [0; CHUNK_SIZE];
//     let num_read = reader.read(&mut buffer)?;
//     Ok(Vec::from(&buffer[..num_read]))
// }

pub fn read_loop(infile: &str, quit: Arc<Mutex<bool>>) -> IoResult<()> {
    let mut reader: Box<dyn Read> = if !infile.is_empty() {
        Box::new(BufReader::new(File::open(infile)?))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };
    let mut buffer = [0; CHUNK_SIZE];
    loop {
        let num_read = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };
        // todo: send the buffer to stats_thread
        Vec::from(&buffer[..num_read]);
    }
    // todo: send empty buffer to stats
    let mut quit = quit.lock().unwrap();
    *quit = true;
    Ok(())
}
