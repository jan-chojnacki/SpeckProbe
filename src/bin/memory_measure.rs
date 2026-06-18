use speck_probe::speck;

#[inline(never)]
fn fn_addr<A, B, R>(f: unsafe fn(A, B) -> R) -> usize {
    f as usize
}

fn main() {
    let _ = std::hint::black_box([
        fn_addr(speck::scalar_encrypt_block_64_128),
        fn_addr(speck::scalar_encrypt_block_inflight_64_128),
        fn_addr(speck::scalar_decrypt_block_64_128),
    ]);

    #[cfg(target_arch = "x86_64")]
    let _ = std::hint::black_box([
        fn_addr(speck::sse2_encrypt_block_64_128),
        fn_addr(speck::sse2_encrypt_block_inflight_64_128),
        fn_addr(speck::sse2_decrypt_block_64_128),
        fn_addr(speck::avx2_encrypt_block_64_128),
        fn_addr(speck::avx2_encrypt_block_inflight_64_128),
        fn_addr(speck::avx2_decrypt_block_64_128),
        fn_addr(speck::avx512_encrypt_block_64_128),
        fn_addr(speck::avx512_encrypt_block_inflight_64_128),
        fn_addr(speck::avx512_decrypt_block_64_128),
    ]);

    #[cfg(target_arch = "aarch64")]
    let _ = std::hint::black_box([
        fn_addr(speck::neon_encrypt_block_64_128),
        fn_addr(speck::neon_encrypt_block_inflight_64_128),
        fn_addr(speck::neon_decrypt_block_64_128),
    ]);
}
