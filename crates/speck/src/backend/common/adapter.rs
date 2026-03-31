#[macro_export]
macro_rules! impl_adapter {
    ($adapter_name:ident, $fn_name:path, $from:ty, $to:ty, $key_words:literal) => {
        #[inline(always)]
        pub fn $adapter_name(ct: [$to; 2], key: [$to; $key_words]) -> [$to; 2]
        where
            $from: From<$to>,
            $to: From<$from>,
        {
            $fn_name(ct.map(<$from>::from), key.map(<$from>::from)).map(<$to>::from)
        }
    };
}
