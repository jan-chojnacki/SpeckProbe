macro_rules! define_search {
    (
    $(#[$meta:meta])*
    version = $version:path,
    bytes = $bytes:literal,
    vector = $vector:ty,
    comparator = $comparator:path,
    key_conversion = $key_conversion:ident,
    new_key = $new_key:ident,
    next_key = $next_key:ident,
    name = $name:tt,
    simd = $simd:tt
    ) => {paste! {
        $(#[$meta])*
        #[doc = "# Safety"]
        #[doc = "Caller must ensure CPU support for `" $simd "` before calling this function."]
        pub fn [<$simd _search_encrypt_ $name>] <const PREFIX: usize>(
            task: $crate::domain::Task<$vector, $bytes, PREFIX>,
            out: &mut Vec<$crate::domain::Key<$bytes, PREFIX>>,
        ) {
            let mut iter = $crate::domain::KeyIterator::<$bytes, PREFIX>::new(task.start, task.end, task.prefix, $version);
            let mut key = iter.$new_key();

            while iter.$next_key(&mut key).is_some() {
                let result = speck::[<$simd _encrypt_block_ $name>](task.data, key.$key_conversion());
                $comparator(&task.expected, &result, &key, out);
            }
        }

        $(#[$meta])*
        #[doc = "# Safety"]
        #[doc = "Caller must ensure CPU support for `" $simd "` before calling this function."]
        pub fn [<$simd _search_encrypt_inflight_ $name>]<const PREFIX: usize>(
            task: $crate::domain::Task<$vector, $bytes, PREFIX>,
            out: &mut Vec<$crate::domain::Key<$bytes, PREFIX>>,
        ) {
            let mut iter = $crate::domain::KeyIterator::<$bytes, PREFIX>::new(task.start, task.end, task.prefix, $version);
            let mut key = iter.$new_key();

            while iter.$next_key(&mut key).is_some() {
                let result = speck::[<$simd _encrypt_block_inflight_ $name>](task.data, key.$key_conversion());
                $comparator(&task.expected, &result, &key, out);
            }
        }

        $(#[$meta])*
        #[doc = "# Safety"]
        #[doc = "Caller must ensure CPU support for `" $simd "` before calling this function."]
        pub fn [<$simd _search_decrypt_ $name>]<const PREFIX: usize>(
            task: $crate::domain::Task<$vector, $bytes, PREFIX>,
            out: &mut Vec<$crate::domain::Key<$bytes, PREFIX>>,
        ) {
            let mut iter = $crate::domain::KeyIterator::<$bytes, PREFIX>::new(task.start, task.end, task.prefix, $version);
            let mut key = iter.$new_key();

            while iter.$next_key(&mut key).is_some() {
                let result = speck::[<$simd _decrypt_block_ $name>](task.data, key.$key_conversion());
                $comparator(&task.expected, &result, &key, out);
            }
        }
    }};
}

pub(crate) use define_search;
