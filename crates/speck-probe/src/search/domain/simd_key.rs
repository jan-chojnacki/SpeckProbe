pub trait SimdKey<const LANES: usize> {
    fn update(&mut self, v: [u64; LANES]);
}
