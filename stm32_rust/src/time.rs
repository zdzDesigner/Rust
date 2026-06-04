pub struct Duration {
    pub micros: u64,
}

pub struct Delay;

impl Delay {
    pub const fn new() -> Self {
        Self
    }

    pub fn cycles(&self, count: u32) {
        let mut i = 0;
        while i < count {
            core::hint::black_box(i);
            i += 1;
        }
    }
}
