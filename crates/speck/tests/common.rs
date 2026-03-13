macro_rules! define_speck_test {
    ($name:literal, $function:path, $arch:literal, $feature:literal, key = $key:expr, data = $data:expr, expected = $expected:expr) => {
        paste::paste! {
            #[test]
            #[cfg(target_arch = $arch)]
            fn [<$name _test>]() {
                if !is_x86_feature_detected!($feature) {
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

pub(crate) use define_speck_test;