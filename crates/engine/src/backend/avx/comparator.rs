use std::arch::x86_64::{__m128i, _mm_storeu_si128};
use std::mem::MaybeUninit;

#[cfg(all(target_arch = "x86_64", target_feature = "avx"))]
#[target_feature(enable = "avx")]
pub fn block_compare<U, const T: usize>(e: &[U; 2], v: [__m128i; 2]) -> Option<Vec<usize>>
where
    U: PartialEq,
{
    debug_assert!(size_of::<U>() * T == 16);

    let v = v.map(|m| {
        let mut out = MaybeUninit::<[U; T]>::uninit();
        unsafe {
            _mm_storeu_si128(out.as_mut_ptr() as *mut __m128i, m);
            out.assume_init()
        }
    });

    let r: Vec<usize> = (0..T)
        .filter(|&i| v[0][i] == e[0] && v[1][i] == e[1])
        .collect();

    match r.is_empty() {
        true => None,
        false => Some(r),
    }
}
