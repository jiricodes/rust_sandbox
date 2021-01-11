use std::fmt;

pub struct Stats {
    total_bytes: usize,
    all_read: bool,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            total_bytes: 0,
            all_read: false,
        }
    }

    pub fn update(&mut self, num_read: usize, all_done: bool) {
        self.total_bytes += num_read;
        self.all_read = all_done;
    }

    pub fn eprint_status(&self) {
        eprint!("\r{}", self);
        if self.all_read {
            eprintln!();
        }
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Total Bytes {}", self.total_bytes)
    }
}