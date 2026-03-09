#[cfg(target_arch = "x86_64")]

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::{
    __m128i, __m256i, __m512i, _mm256_add_epi32, _mm256_rol_epi32, _mm256_ror_epi32, _mm256_set1_epi32,
    _mm256_setzero_si256, _mm256_sub_epi32, _mm256_xor_epi32, _mm512_add_epi32, _mm512_rol_epi32,
    _mm512_ror_epi32, _mm512_set1_epi32, _mm512_setzero_si512, _mm512_sub_epi32, _mm512_xor_epi32,
    _mm_add_epi32, _mm_rol_epi32, _mm_ror_epi32, _mm_set1_epi32, _mm_setzero_si128,
    _mm_sub_epi32, _mm_xor_epi32,
};

#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::{
    uint32x4_t, vaddq_u32, veorq_u32, vorrq_u32, vshlq_n_u32, vshrq_n_u32,
};

#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::{uint16x8_t, uint64x2_t};

#[cfg(target_arch = "aarch64")]
#[inline(always)]
fn encrypt_round_neon(x: &mut uint32x4_t, y: &mut uint32x4_t, k: uint32x4_t) {
    unsafe {
        // x = (x ror 8) + y ^ k
        let xr = vorrq_u32(
            vshrq_n_u32(*x, 8 as i32),
            vshlq_n_u32(*x, (32 - 8) as i32),
        );
        *x = veorq_u32(vaddq_u32(xr, *y), k);

        // y = (y rol 3) ^ x
        let yl = vorrq_u32(
            vshlq_n_u32(*y, 3 as i32),
            vshrq_n_u32(*y, (32 - 3) as i32),
        );
        *y = veorq_u32(yl, *x);
    }
}

mod encrypt_round;
mod decrypt_round;
mod operations;
mod expand_key;
mod decrypt_block;
mod encrypt_block;

mod backend;

pub use backend::*;

// fn encrypt_round(x: &mut u32, y: &mut u32, k: u32) {
//     *x = x.rotate_right(ALPHA).wrapping_add(*y).bitxor(k);
//     *y = y.rotate_left(BETA).bitxor(*x);
// }

#[cfg(target_arch = "aarch64")]
macro_rules! neon_word_ty {
    (16) => {
        uint16x8_t
    };
    (32) => {
        uint32x4_t
    };
    (64) => {
        uint64x2_t
    };
}

#[cfg(target_arch = "aarch64")]
pub(crate) use neon_word_ty;



#[cfg(target_arch = "x86_64")]
fn encrypt_round_avx2(x: &mut __m256i, y: &mut __m256i, k: __m256i) {
    unsafe {
        *x = _mm256_ror_epi32(*x, 8 as i32);
        *x = _mm256_add_epi32(*x, *y);
        *x = _mm256_xor_epi32(*x, k);

        *y = _mm256_rol_epi32(*y, 3 as i32);
        *y = _mm256_xor_epi32(*y, *x);
    }
}

#[cfg(target_arch = "x86_64")]
fn encrypt_round_avx512(x: &mut __m512i, y: &mut __m512i, k: __m512i) {
    unsafe {
        *x = _mm512_ror_epi32(*x, 8 as i32);
        *x = _mm512_add_epi32(*x, *y);
        *x = _mm512_xor_epi32(*x, k);

        *y = _mm512_rol_epi32(*y, 3 as i32);
        *y = _mm512_xor_epi32(*y, *x);
    }
}



#[cfg(target_arch = "x86_64")]
fn decrypt_round_avx2(x: &mut __m256i, y: &mut __m256i, k: __m256i) {
    unsafe {
        *y = _mm256_xor_epi32(*y, *x);
        *y = _mm256_ror_epi32(*y, 3 as i32);

        *x = _mm256_xor_epi32(*x, k);
        *x = _mm256_sub_epi32(*x, *y);
        *x = _mm256_rol_epi32(*x, 8 as i32);
    }
}

#[cfg(target_arch = "x86_64")]
fn decrypt_round_avx512(x: &mut __m512i, y: &mut __m512i, k: __m512i) {
    unsafe {
        *y = _mm512_xor_epi32(*y, *x);
        *y = _mm512_ror_epi32(*y, 3 as i32);

        *x = _mm512_xor_epi32(*x, k);
        *x = _mm512_sub_epi32(*x, *y);
        *x = _mm512_rol_epi32(*x, 8 as i32);
    }
}



#[cfg(target_arch = "x86_64")]
fn expand_key_avx2(key: [__m256i; 4]) -> [__m256i; 27] {
    unsafe {
        let mut round_keys: [__m256i; 27] = [_mm256_setzero_si256(); 27];

        let mut l2 = key[0];
        let mut l1 = key[1];
        let mut l0 = key[2];
        let mut k0 = key[3];

        let mut i: usize = 0;
        while i < 27 - 1 {
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

#[cfg(target_arch = "x86_64")]
fn expand_key_avx512(key: [__m512i; 4]) -> [__m512i; 27] {
    unsafe {
        let mut round_keys: [__m512i; 27] = [_mm512_setzero_si512(); 27];

        let mut l2 = key[0];
        let mut l1 = key[1];
        let mut l0 = key[2];
        let mut k0 = key[3];

        let mut i: usize = 0;
        while i < 27 - 1 {
            round_keys[i] = k0;
            encrypt_round_avx512(&mut l0, &mut k0, _mm512_set1_epi32(i as i32));
            i += 1;

            round_keys[i] = k0;
            encrypt_round_avx512(&mut l1, &mut k0, _mm512_set1_epi32(i as i32));
            i += 1;

            round_keys[i] = k0;
            encrypt_round_avx512(&mut l2, &mut k0, _mm512_set1_epi32(i as i32));
            i += 1;
        }

        round_keys
    }
}



#[cfg(target_arch = "x86_64")]
pub fn encrypt_block_avx2(pt: [__m256i; 2], key: [__m256i; 4]) -> [__m256i; 2] {
    let round_keys = expand_key_avx2(key);

    let mut x = pt[0];
    let mut y = pt[1];

    for &k in &round_keys {
        encrypt_round_avx2(&mut x, &mut y, k);
    }

    [x, y]
}

#[cfg(target_arch = "x86_64")]
pub fn encrypt_block_avx512(pt: [__m512i; 2], key: [__m512i; 4]) -> [__m512i; 2] {
    let round_keys = expand_key_avx512(key);

    let mut x = pt[0];
    let mut y = pt[1];

    for &k in &round_keys {
        encrypt_round_avx512(&mut x, &mut y, k);
    }

    [x, y]
}



#[cfg(target_arch = "x86_64")]
pub fn decrypt_block_avx2(ct: [__m256i; 2], key: [__m256i; 4]) -> [__m256i; 2] {
    let round_keys = expand_key_avx2(key);

    let mut x = ct[0];
    let mut y = ct[1];

    for &k in round_keys.iter().rev() {
        decrypt_round_avx2(&mut x, &mut y, k);
    }

    [x, y]
}

#[cfg(target_arch = "x86_64")]
pub fn decrypt_block_avx512(ct: [__m512i; 2], key: [__m512i; 4]) -> [__m512i; 2] {
    let round_keys = expand_key_avx512(key);

    let mut x = ct[0];
    let mut y = ct[1];

    for &k in round_keys.iter().rev() {
        decrypt_round_avx512(&mut x, &mut y, k);
    }

    [x, y]
}
