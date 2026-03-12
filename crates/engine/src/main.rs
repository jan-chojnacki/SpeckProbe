use speck::{decrypt_block_64_128, encrypt_block_64_128};

#[cfg(target_arch = "x86_64")]
use speck::{avx2_decrypt_block_64_128, avx2_encrypt_block_64_128};
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::_mm256_set1_epi32;

fn main() {
    // Key:         1b1a1918 13121110 0b0a0908 03020100
    // Plaintext:   3b726574 7475432d
    // Ciphertext:  8c6fa548 454e028b

    let key = [0x1b1a1918, 0x13121110, 0x0b0a0908, 0x03020100];
    let pt = [0x3b726574, 0x7475432d];

    let ct = encrypt_block_64_128(pt, key);
    let pt = decrypt_block_64_128(ct, key);

    println!("ciphertext = {:08x} {:08x}", ct[0], ct[1]);
    println!("plaintext = {:08x} {:08x}", pt[0], pt[1]);

    #[cfg(target_arch = "x86_64")]
    unsafe {
        let key_avx = [
            _mm256_set1_epi32(0x1b1a1918),
            _mm256_set1_epi32(0x13121110),
            _mm256_set1_epi32(0x0b0a0908),
            _mm256_set1_epi32(0x03020100),
        ];

        let pt_avx = [_mm256_set1_epi32(0x3b726574), _mm256_set1_epi32(0x7475432d)];

        let ct_avx = avx2_encrypt_block_64_128(pt_avx, key_avx);
        let pt_avx = avx2_decrypt_block_64_128(ct_avx, key_avx);

        println!(
            "ciphertext_avx2[0] = {:08x?}\nciphertext_avx2[1] = {:08x?}",
            ct_avx[0], ct_avx[1]
        );
        println!(
            "plaintext_avx2[0] = {:08x?}\nplaintext_avx2[1] = {:08x?}",
            pt_avx[0], pt_avx[1]
        );
    }
}
