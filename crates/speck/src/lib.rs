mod backend;

pub use backend::*;

macro_rules! define_speck_test {
    ($function:ident, $arch:literal, $feature:literal, key = $key:expr, data = $data:expr, expected = $expected:expr) => {
        paste::paste! {
            #[test]
            #[cfg(target_arch = $arch)]
            fn [<$function _test>]() {
                if !is_x86_feature_detected!($feature) {
                    return;
                }
                unsafe { [<$function _test_unsafe>]() }
            }

            #[cfg(all(target_arch = $arch, target_feature = $feature))]
            #[target_feature(enable = $feature)]
            unsafe fn [<$function _test_unsafe>]() {
                let key = $key;
                let data = $data;
                let expected = $expected;

                let result = avx_encrypt_block_64_128(data, key);

                assert_eq!(bytemuck::bytes_of(&result), bytemuck::bytes_of(&expected));
            }
        }
    };
}

pub(crate) use define_speck_test;