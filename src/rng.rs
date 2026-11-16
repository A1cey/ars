/// This is an implementation of the PCG RNG generator.
/// <https://www.pcg-random.org>
#[derive(Debug, Default)]
pub struct PCGRand {
    state: u64,
    inc: u64,
}

impl PCGRand {
    /// Returns the next random value.
    pub const fn random(&mut self) -> u64 {
        let old_state = self.state;
        self.state = old_state * 6_364_136_223_846_793_005 + (self.inc | 1);
        let xor_shifted = ((old_state >> 18) ^ old_state) >> 27;
        let rot = old_state >> 59;
        (xor_shifted >> rot) | (xor_shifted << (rot.wrapping_neg() & 31))
    }
}
