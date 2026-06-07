// macro_rules! define_speck_memory_test {
//     ($name:literal, $function:path, key = $key:expr, data = $data:expr) => {
//         paste::paste! {
//             #[test]
//             fn [<$name _test_unsafe>]() {
//                 let key = $key;
//                 let data = $data;
//
//                 let result = $function(data, key);
//
//                 std::hint::black_box(&result);
//             }
//         }
//     };
// }

#[cfg(test)]
mod test {
    #[global_allocator]
    static ALLOC: dhat::Alloc = dhat::Alloc;

    #[test]
    fn scalar_memory_budget() {
        let _profiler = dhat::Profiler::builder().testing().build();

        let key = [0x1918, 0x1110, 0x0908, 0x0100];
        let data = [0x6574, 0x694c];

        let result = crate::speck::scalar_encrypt_block_32_64(data, key);

        std::hint::black_box(&result);

        let stats = dhat::HeapStats::get();
        eprintln!(
            "total: {} blocks / {} bytes, peak: {} blocks / {} bytes",
            stats.total_blocks, stats.total_bytes, stats.max_blocks, stats.max_bytes
        );
    }
}
