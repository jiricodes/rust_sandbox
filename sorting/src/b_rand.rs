use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref RG: Mutex<RandGen> = Mutex::new(RandGen::new(34052));
}

pub fn rand(max: usize) -> usize {
    RG.lock().unwrap().next_v(max)
}

pub struct RandGen {
    cur: usize,
    mul: usize,
    inc: usize,
    modulo: usize,
}

impl RandGen {
    pub fn new(cur: usize) -> Self {
        Self {
            cur,
            mul: 56394237,
            inc: 34642349,
            modulo: 25964951,
        }
    }

    pub fn next_v(&mut self, max: usize) -> usize {
        self.cur = (self.cur * self.mul + self.inc) % self.modulo;
        self.cur % max
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rands_print() {
        let mut r = RandGen::new(12);
        for _ in 0..100 {
            println!("--{}", r.next_v(1000));
        }
        // panic!();
    }
}
