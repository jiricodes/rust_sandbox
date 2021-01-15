//! The stats module contains the stats loop that is performed on stats thread.
//! 
use crossbeam::channel::Receiver;
use crossterm::{
    cursor, execute,
    style::{self, Color, PrintStyledContent},
    terminal::{Clear, ClearType},
};
use std::fmt;
use std::io::{self, Result as IoResult, Stderr, Write};
use std::time::{Duration, Instant};

pub struct Stats {
    total_bytes: usize,
    bps: f64,
    start: Instant,
    last_instant: Instant,
    delta: Duration,
    period: Duration,
    countdown: Duration,
    ready: bool,
}

impl Stats {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            total_bytes: 0,
            bps: 0.0,
            start: now,
            last_instant: now,
            delta: Duration::default(),
            period: Duration::from_millis(1000),
            countdown: Duration::default(),
            ready: true,
        }
    }

    pub fn update(&mut self, num_read: usize) {
        self.total_bytes += num_read;
        let now = Instant::now();
        self.delta = now - self.last_instant;
        self.last_instant = now;
        self.countdown = self.countdown.checked_sub(self.delta).unwrap_or_else(|| {
            self.ready = true;
            self.period
        });
        self.bps = num_read as f64 / self.delta.as_secs_f64();
    }

    pub fn print(&mut self, stderr: &mut Stderr) {
        let mut bytes = self.total_bytes;
        let mut units = "b";
        if self.total_bytes > (1024 * 1024 * 1024 * 1024) {
            bytes /= 1024 * 1024 * 1024 * 1024;
            units = "Tb";
        } else if self.total_bytes > (1024 * 1024 * 1024) {
            bytes /= 1024 * 1024 * 1024;
            units = "Gb";
        } else if self.total_bytes > (1024 * 1024) {
            bytes /= 1024 * 1024;
            units = "Mb";
        } else if self.total_bytes > 1024 {
            bytes /= 1024;
            units = "Kb";
        }
        let bytes = style::style(format!("{} {} ", bytes, units)).with(Color::Red);
        let elapsed = style::style(self.start.elapsed().as_secs().as_time()).with(Color::Green);
        let mut bps = self.bps;
        let mut units = "b/s";
        if self.bps > (1024.0 * 1024.0) {
            bps /= 1024.0 * 1024.0;
            units = "Mb/s";
        } else if self.bps > 1024.0 {
            bps /= 1024.0;
            units = "Kb/s";
        }
        let rate = style::style(format!(" [{:.0} {}]", bps, units)).with(Color::Blue);
        let _ = execute!(
            stderr,
            cursor::MoveToColumn(0),
            Clear(ClearType::CurrentLine),
            PrintStyledContent(bytes),
            PrintStyledContent(elapsed),
            PrintStyledContent(rate)
        );
        let _ = stderr.flush();
        self.ready = false;
    }
}

/// The TimeOutput trait adds a `.as_time()` method to `u64`
/// 
/// # Example
/// Here us an example.
/// ```rust
/// use pipeviewer::stats::TimeOutput
/// assert_eq!(12345_u64.as_time(), String::from("3:25:45"));
/// ```
pub trait TimeOutput {
    fn as_time(&self) -> String;
}

impl TimeOutput for u64 {
    /// Renders the u64 into a time string
    fn as_time(&self) -> String {
        let (hours, left) = (*self / 3600, *self % 3600);
        let (minutes, seconds) = (left / 60, left % 60);
        format!("{}:{:02}:{:02}", hours, minutes, seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::TimeOutput;

    #[test]
    fn as_time_format() {
        let pairs = vec![
            (5_u64, "0:00:05"),
            (60_u64, "0:01:00"),
            (3600_u64, "1:00:00"),
            (85_u64, "0:01:25"),
            (12345_u64, "3:25:45"),
            (3599_u64, "0:59:59"),
        ];
        for (input, output) in pairs {
            assert_eq!(input.as_time().as_str(), output);
        }
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut bps = self.bps;
        let mut units = "b/s";
        if self.bps > (1024.0 * 1024.0) {
            bps /= 1024.0 * 1024.0;
            units = "mb/s";
        } else if self.bps > 1024.0 {
            bps /= 1024.0;
            units = "kb/s";
        }
        write!(
            f,
            "Total Bytes {} {}s [{:.0} {}]",
            self.total_bytes,
            self.start.elapsed().as_secs().as_time(),
            bps,
            units
        )
    }
}

pub fn stats_loop(silent: bool, stats_rx: Receiver<usize>) -> IoResult<()> {
    let mut timer = Stats::new();
    let mut stderr = io::stderr();
    loop {
        let num_bytes = stats_rx.recv().unwrap();
        timer.update(num_bytes);
        if num_bytes == 0 {
            break;
        }
        if !silent && timer.ready {
            timer.print(&mut stderr);
            // eprint!("\r{}", timer);
            // timer.ready = false;
        }
    }
    if !silent {
        eprintln!();
    }
    Ok(())
}
