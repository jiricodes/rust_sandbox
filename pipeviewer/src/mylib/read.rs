use super::CHUNK_SIZE;
use crossbeam::channel::Sender;
use std::fs::File;
use std::io::{self, BufReader, Read, Result as IoResult};

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

pub fn read_loop(infile: &str, stats_tx: Sender<usize>, write_tx: Sender<Vec<u8>>) -> IoResult<()> {
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
        let _ = stats_tx.send(num_read);
        // todo: send the buffer to stats_thread
        if write_tx.send(Vec::from(&buffer[..num_read])).is_err() {
            break;
        }
    }
    // todo: send empty buffer to stats
    let _ = stats_tx.send(0);
    let _ = write_tx.send(Vec::new());
    Ok(())
}
