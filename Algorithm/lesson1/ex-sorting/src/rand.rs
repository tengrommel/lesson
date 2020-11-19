use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref RG: Mutex<RandGen> = Mutex::new(RandGen::new(34052));
}

pub fn read(max: usize) -> usize {
    RG.lock().unwrap().next(max)
}

struct RandGen {
    curr: usize,
    mul: usize,
    inc: usize,
    modulo: usize
}

impl RandGen {
    fn new(curr: usize) ->Self {
        RandGen{
            curr,
            mul: 56394237,
            inc: 34642349,
            modulo: 23254544563,
        }
    }
    fn next(&mut self, max: usize) -> usize {
        self.curr = (self.curr * self.mul + self.inc)%self.modulo;
        self.curr % max
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_rands_out() {
        let mut r = RandGen::new(12);
        for _ in 0..100 {
            println!("-- {}", r.next(100));
        }
    }
}