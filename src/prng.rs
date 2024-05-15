pub struct Prng {
    state: u64,
}

impl Prng {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    // https://en.wikipedia.org/wiki/Xorshift
    pub fn random_u64(&mut self) -> u64 {
        let mut result = self.state;

        result ^= result >> 12;
        result ^= result << 25;
        result ^= result >> 27;

        self.state = result;
        result * 2685821657736338717u64
    }

    pub fn sparse_random_u64(&mut self) -> u64 {
        self.random_u64() & self.random_u64() & self.random_u64()
    }
}
