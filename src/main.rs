use std::arch::x86_64::*;
use std::ops::BitXor;

const ALPHA: u32 = 8;
const BETA: u32 = 3;
const ROUNDS: usize = 27;

fn encrypt_round(x: &mut u32, y: &mut u32, k: u32) {
    *x = x.rotate_right(ALPHA).wrapping_add(*y).bitxor(k);
    *y = y.rotate_left(BETA).bitxor(*x);
}

fn encrypt_round_avx2(x: &mut __m256i, y: &mut __m256i, k: __m256i) {
    unsafe {
        *x = _mm256_ror_epi32(*x, ALPHA as i32);
        *x = _mm256_add_epi32(*x, *y);
        *x = _mm256_xor_epi32(*x, k);

        *y = _mm256_rol_epi32(*y, BETA as i32);
        *y = _mm256_xor_epi32(*y, *x);
    }
}

fn decrypt_round(x: &mut u32, y: &mut u32, k: u32) {
    *y = y.bitxor(*x).rotate_right(BETA);
    *x = x.bitxor(k).wrapping_sub(*y).rotate_left(ALPHA);
}

fn decrypt_round_avx2(x: &mut __m256i, y: &mut __m256i, k: __m256i) {
    unsafe {
        *y = _mm256_xor_epi32(*y, *x);
        *y = _mm256_ror_epi32(*y, BETA as i32);

        *x = _mm256_xor_epi32(*x, k);
        *x = _mm256_sub_epi32(*x, *y);
        *x = _mm256_rol_epi32(*x, ALPHA as i32);
    }
}

fn expand_key(key: [u32; 4]) -> [u32; ROUNDS] {
    let mut round_keys: [u32; ROUNDS] = [0; ROUNDS];

    let mut l2 = key[0];
    let mut l1 = key[1];
    let mut l0 = key[2];
    let mut k0 = key[3];

    let mut i: usize = 0;
    while i < ROUNDS - 1 {
        round_keys[i] = k0;
        encrypt_round(&mut l0, &mut k0, i as u32);
        i += 1;

        round_keys[i] = k0;
        encrypt_round(&mut l1, &mut k0, i as u32);
        i += 1;

        round_keys[i] = k0;
        encrypt_round(&mut l2, &mut k0, i as u32);
        i += 1;
    }

    round_keys
}

fn expand_key_avx2(key: [__m256i; 4]) -> [__m256i; ROUNDS] {
    unsafe {
        let mut round_keys: [__m256i; ROUNDS] = [_mm256_setzero_si256(); ROUNDS];

        let mut l2 = key[0];
        let mut l1 = key[1];
        let mut l0 = key[2];
        let mut k0 = key[3];

        let mut i: usize = 0;
        while i < ROUNDS - 1 {
            round_keys[i] = k0;
            encrypt_round_avx2(&mut l0, &mut k0, _mm256_set1_epi32(i as i32));
            i += 1;

            round_keys[i] = k0;
            encrypt_round_avx2(&mut l1, &mut k0, _mm256_set1_epi32(i as i32));
            i += 1;

            round_keys[i] = k0;
            encrypt_round_avx2(&mut l2, &mut k0, _mm256_set1_epi32(i as i32));
            i += 1;
        }

        round_keys
    }
}

fn encrypt_block(pt: [u32; 2], key: [u32; 4]) -> [u32; 2] {
    let round_keys = expand_key(key);

    let mut x = pt[0];
    let mut y = pt[1];

    for &k in &round_keys {
        encrypt_round(&mut x, &mut y, k);
    }

    [x, y]
}

fn encrypt_block_avx2(pt: [__m256i; 2], key: [__m256i; 4]) -> [__m256i; 2] {
    let round_keys = expand_key_avx2(key);

    let mut x = pt[0];
    let mut y = pt[1];

    for &k in &round_keys {
        encrypt_round_avx2(&mut x, &mut y, k);
    }

    [x, y]
}

fn decrypt_block(ct: [u32; 2], key: [u32; 4]) -> [u32; 2] {
    let round_keys = expand_key(key);

    let mut x = ct[0];
    let mut y = ct[1];

    for &k in round_keys.iter().rev() {
        decrypt_round(&mut x, &mut y, k);
    }

    [x, y]
}

fn decrypt_block_avx2(ct: [__m256i; 2], key: [__m256i; 4]) -> [__m256i; 2] {
    let round_keys = expand_key_avx2(key);

    let mut x = ct[0];
    let mut y = ct[1];

    for &k in round_keys.iter().rev() {
        decrypt_round_avx2(&mut x, &mut y, k);
    }

    [x, y]
}

fn main() {
    // Key:         1b1a1918 13121110 0b0a0908 03020100
    // Plaintext:   3b726574 7475432d
    // Ciphertext:  8c6fa548 454e028b

    let key = [0x1b1a1918, 0x13121110, 0x0b0a0908, 0x03020100];
    let pt = [0x3b726574, 0x7475432d];

    let ct = encrypt_block(pt, key);
    let pt = decrypt_block(ct, key);

    println!("ciphertext = {:08x} {:08x}", ct[0], ct[1]);
    println!("plaintext = {:08x} {:08x}", pt[0], pt[1]);

    unsafe {
        let key_avx = [
            _mm256_set1_epi32(0x1b1a1918),
            _mm256_set1_epi32(0x13121110),
            _mm256_set1_epi32(0x0b0a0908),
            _mm256_set1_epi32(0x03020100),
        ];

        let pt_avx = [_mm256_set1_epi32(0x3b726574), _mm256_set1_epi32(0x7475432d)];

        let ct_avx = encrypt_block_avx2(pt_avx, key_avx);
        let pt_avx = decrypt_block_avx2(ct_avx, key_avx);

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
