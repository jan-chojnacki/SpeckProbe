pub trait SimdKey<const T: usize> {
    fn update(&mut self, v: [u64; T]);
}
