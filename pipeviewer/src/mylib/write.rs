use crossbeam::channel::Receiver;
use std::fs::File;
use std::io::{self, BufWriter, ErrorKind, Result as IoResult, Write};

// pub fn write_output(outfile: &str, buffer: &[u8]) -> IoResult<bool> {
//     let mut writer: Box<dyn Write> = if !outfile.is_empty() {
//         Box::new(BufWriter::new(File::create(outfile)?))
//     } else {
//         Box::new(BufWriter::new(io::stdout()))
//     };

//     if let Err(e) = writer.write_all(&buffer) {
//         if e.kind() == ErrorKind::BrokenPipe {
//             //false means "stop program cleanly"
//             return Ok(false);
//         }
//         return Err(e);
//     }
//     // true means "keep going"
//     Ok(true)
// }

pub fn write_loop(outfile: &str, write_rx: Receiver<Vec<u8>>) -> IoResult<()> {
    let mut writer: Box<dyn Write> = if !outfile.is_empty() {
        Box::new(BufWriter::new(File::create(outfile)?))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };

    loop {
        let buffer: Vec<u8> = write_rx.recv().unwrap();
        if buffer.is_empty() {
            break;
        }
        if let Err(e) = writer.write_all(&buffer) {
            if e.kind() == ErrorKind::BrokenPipe {
                //stop program cleanly
                return Ok(());
            }
            return Err(e);
        }
    }
    Ok(())
}
