use std::fs::File;
use std::io::{self, BufWriter, ErrorKind, Result as IoResult, Write};
use std::sync::{Arc, Mutex};

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

pub fn write_loop(outfile: &str, quit: Arc<Mutex<bool>>) -> IoResult<()> {
    let mut writer: Box<dyn Write> = if !outfile.is_empty() {
        Box::new(BufWriter::new(File::create(outfile)?))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };

    loop {
        //todo: receive buffer from stats thread
        let buffer: Vec<u8> = Vec::new();
        {
            let quit = quit.lock().unwrap();
            if *quit {
                break;
            }
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
