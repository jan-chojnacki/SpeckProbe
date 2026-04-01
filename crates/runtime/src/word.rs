pub trait Word: Copy + Clone + Send + Sync + PartialEq + 'static {}
impl Word for u16 {}
impl Word for u32 {}
impl Word for u64 {}
