mod mylib;
use mylib::{args::Args, read, stats::Stats, write};
use std::io::Result as IoResult;

fn main() -> IoResult<()> {
    let args = Args::new();
    let mut stats = Stats::new();
    loop {
        let buffer = match read::read_input(&args.infile) {
            Ok(x) if x.is_empty() => break,
            Ok(x) => x,
            Err(_) => break,
        };
        stats.update(buffer.len(), false);
        if !args.silent {
            stats.eprint_status();
        }
        if !write::write_output(&args.outfile, &buffer)? {
            break;
        }
    }
    stats.update(0, true);
    if !args.silent {
        stats.eprint_status();
    }
    Ok(())
}
