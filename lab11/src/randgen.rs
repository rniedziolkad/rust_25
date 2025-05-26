pub struct RandGen {
    seed: i64
}

impl RandGen {
    pub fn new(seed: i64) -> Self {
        Self { seed }
    }

    pub fn gen_range(&mut self, min: i64, max: i64) -> i64 {
        const A: i64 = 1664525;
        const C: i64 = 1013904223;
        const M: i64 = 1<<32;
        self.seed = (self.seed.wrapping_mul(A).wrapping_add(C)) % M;
        min + (self.seed % (max-min+1))
    }
}