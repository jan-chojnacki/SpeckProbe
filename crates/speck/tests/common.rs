#[allow(unused_macros)]
macro_rules! define_speck_test {
    ($name:literal, $function:path, key = $key:expr, data = $data:expr, expected = $expected:expr) => {
        paste::paste! {
            #[test]
            fn [<$name _test_unsafe>]() {
                let key = $key;
                let data = $data;
                let expected = $expected;

                let result = $function(data, key);

                assert_eq!(bytemuck::bytes_of(&result), bytemuck::bytes_of(&expected));
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! define_speck_test_x86_64_simd {
    ($name:literal, $function:path, $arch:literal, $feature:literal, key = $key:expr, data = $data:expr, expected = $expected:expr) => {
        paste::paste! {
            #[test]
            #[cfg(target_arch = "x86_64")]
            fn [<$name _test>]() {
                if !std::x86_64::is_x86_feature_detected!($feature) {
                    return;
                }
                unsafe { [<$name _test_unsafe>]() }
            }

            #[cfg(all(target_arch = $arch, target_feature = $feature))]
            #[target_feature(enable = $feature)]
            unsafe fn [<$name _test_unsafe>]() {
                let key = $key;
                let data = $data;
                let expected = $expected;

                let result = $function(data, key);

                assert_eq!(bytemuck::bytes_of(&result), bytemuck::bytes_of(&expected));
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! define_speck_test_aarch64_simd {
    ($name:literal, $function:path, $arch:literal, $feature:literal, key = $key:expr, data = $data:expr, expected = $expected:expr) => {
        paste::paste! {
            #[test]
            #[cfg(target_arch = "aarch64")]
            fn [<$name _test>]() {
                if !std::arch::is_aarch64_feature_detected!($feature) {
                    return;
                }
                unsafe { [<$name _test_unsafe>]() }
            }

            #[cfg(all(target_arch = $arch, target_feature = $feature))]
            #[target_feature(enable = $feature)]
            unsafe fn [<$name _test_unsafe>]() {
                let key = $key;
                let data = $data;
                let expected = $expected;

                let result = $function(data, key);

                assert_eq!(bytemuck::bytes_of(&result), bytemuck::bytes_of(&expected));
            }
        }
    };
}

#[allow(unused_imports)]
pub(crate) use define_speck_test;
#[allow(unused_imports)]
pub(crate) use define_speck_test_x86_64_simd;
#[allow(unused_imports)]
pub(crate) use define_speck_test_aarch64_simd;
