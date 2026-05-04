macro_rules! define_backend_dispatch {
    (
        $(attrs = [$($attr:meta),* $(,)?],)?
        simd = $simd_prefix:ident,
        versions = [
            $( ($version:tt, bytes=$bytes:literal, ew=$ew:ty, vw=$vw:ty, converter=$conv:expr) ),+
            $(,)?
        ]
        $(,)?
    ) => {
        seq_macro::seq!(S in 1..=4 {
            paste::paste! {
                $($(#[$attr])*)?
                pub(crate) fn dispatch(
                    request: $crate::search::executor::RuntimeRequest,
                    version: $crate::speck::SpeckVersion,
                    suffix: usize,
                    function: $crate::search::executor::CipherFunction,
                ) -> $crate::search::executor::DispatchOutput {
                    use $crate::search::executor::backend::runner::run_orchestrator;
                    match (version, suffix, function) {
                        $(
                            #(
                                ($crate::speck::SpeckVersion::[<Speck $version>], S, $crate::search::executor::CipherFunction::Encrypt) =>
                                    run_orchestrator::<$ew, $vw, $bytes, {$bytes - S}, _, _>(
                                        request,
                                        |task, out| $crate::search::executor::backend::[<$simd_prefix search_encrypt_ $version>](task, out),
                                        $crate::search::executor::backend::[<ecb_validate_encrypt_ $version>],
                                        |__block| ($conv)(__block),
                                    ),
                                ($crate::speck::SpeckVersion::[<Speck $version>], S, $crate::search::executor::CipherFunction::Decrypt) =>
                                    run_orchestrator::<$ew, $vw, $bytes, {$bytes - S}, _, _>(
                                        request,
                                        |task, out| $crate::search::executor::backend::[<$simd_prefix search_decrypt_ $version>](task, out),
                                        $crate::search::executor::backend::[<ecb_validate_encrypt_ $version>],
                                        |__block| ($conv)(__block),
                                    ),
                                ($crate::speck::SpeckVersion::[<Speck $version>], S, $crate::search::executor::CipherFunction::EncryptInflight) =>
                                    run_orchestrator::<$ew, $vw, $bytes, {$bytes - S}, _, _>(
                                        request,
                                        |task, out| $crate::search::executor::backend::[<$simd_prefix search_encrypt_inflight_ $version>](task, out),
                                        $crate::search::executor::backend::[<ecb_validate_encrypt_ $version>],
                                        |__block| ($conv)(__block),
                                    ),
                            )*
                        )+
                        _ => unreachable!("validate suffix/version/function before dispatch"),
                    }
                }
            }
        });
    };
}

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
        pub fn [<$simd _search_encrypt_ $name>] <const PREFIX: usize>(
            task: $crate::search::domain::Task<$vector, $bytes, PREFIX>,
            out: &mut Vec<$crate::search::domain::Key<$bytes, PREFIX>>,
        ) {
            let mut iter = $crate::search::domain::KeyIterator::<$bytes, PREFIX>::new(task.start, task.end, task.prefix, $version);
            let mut key = iter.$new_key();

            while iter.$next_key(&mut key).is_some() {
                let result = $crate::speck::[<$simd _encrypt_block_ $name>](task.data, key.$key_conversion());
                $comparator(&task.expected, &result, &key, out);
            }
        }

        $(#[$meta])*
        pub fn [<$simd _search_encrypt_inflight_ $name>]<const PREFIX: usize>(
            task: $crate::search::domain::Task<$vector, $bytes, PREFIX>,
            out: &mut Vec<$crate::search::domain::Key<$bytes, PREFIX>>,
        ) {
            let mut iter = $crate::search::domain::KeyIterator::<$bytes, PREFIX>::new(task.start, task.end, task.prefix, $version);
            let mut key = iter.$new_key();

            while iter.$next_key(&mut key).is_some() {
                let result = $crate::speck::[<$simd _encrypt_block_inflight_ $name>](task.data, key.$key_conversion());
                $comparator(&task.expected, &result, &key, out);
            }
        }

        $(#[$meta])*
        pub fn [<$simd _search_decrypt_ $name>]<const PREFIX: usize>(
            task: $crate::search::domain::Task<$vector, $bytes, PREFIX>,
            out: &mut Vec<$crate::search::domain::Key<$bytes, PREFIX>>,
        ) {
            let mut iter = $crate::search::domain::KeyIterator::<$bytes, PREFIX>::new(task.start, task.end, task.prefix, $version);
            let mut key = iter.$new_key();

            while iter.$next_key(&mut key).is_some() {
                let result = $crate::speck::[<$simd _decrypt_block_ $name>](task.data, key.$key_conversion());
                $comparator(&task.expected, &result, &key, out);
            }
        }
    }};
}

macro_rules! impl_x86_simd_key {
    ($name:ident, $align:expr, $feature:tt, $vector:ty, $vector_size:expr, $set_zero:path, $load:path) => {
        const VECTOR_SIZE: usize = $vector_size;
        const LANES_U16: usize = VECTOR_SIZE / 16;
        const LANES_U32: usize = VECTOR_SIZE / 32;
        const LANES_U64: usize = VECTOR_SIZE / 64;

        #[repr(align($align))]
        struct Align<T>(T);

        #[derive(Debug, Copy, Clone)]
        #[repr(C, align($align))]
        pub struct $name<const LANES: usize, const BYTES: usize, const PREFIX: usize> {
            bytes: [[u8; BYTES]; LANES],
            pb: $vector,
            pc: $vector,
            pd: $vector,
        }

        impl<const LANES: usize, const BYTES: usize, const PREFIX: usize> $crate::search::domain::simd_key::SimdKey<LANES>
            for $name<LANES, BYTES, PREFIX>
        {
            fn update(&mut self, v: [u64; LANES]) {
                self.update(v);
            }
        }

        impl<const LANES: usize, const BYTES: usize, const PREFIX: usize> $name<LANES, BYTES, PREFIX> {
            const SUFFIX: usize = BYTES - PREFIX;

            #[cfg(target_arch = "x86_64")]
            #[target_feature(enable = $feature)]
            #[doc = "# Safety"]
            #[doc = concat!("Caller must ensure CPU support for `", $feature, "` before calling this function.")]
            pub fn new(prefix: &[u8; PREFIX], v: [u64; LANES], speck_version: $crate::speck::SpeckVersion) -> Self {
                let mut bytes = [[0u8; BYTES]; LANES];

                for i in 0..LANES {
                    bytes[i][Self::SUFFIX..].copy_from_slice(prefix);
                    let suffix = v[i].to_le_bytes();
                    bytes[i][..Self::SUFFIX].copy_from_slice(&suffix[..Self::SUFFIX]);
                }

                let mut pb = $set_zero();
                let mut pc = $set_zero();
                let mut pd = $set_zero();

                match speck_version {
                    $crate::speck::SpeckVersion::Speck32_64 => {
                        if Self::SUFFIX < 2 {
                            let b = Align(bytes.map(|b| [b[2], b[3]]));
                            pb = unsafe { $load(b.0.as_ptr().cast()) }
                        }
                        if Self::SUFFIX < 4 {
                            let c = Align(bytes.map(|b| [b[4], b[5]]));
                            pc = unsafe { $load(c.0.as_ptr().cast()) }
                        }
                        if Self::SUFFIX < 6 {
                            let d = Align(bytes.map(|b| [b[6], b[7]]));
                            pd = unsafe { $load(d.0.as_ptr().cast()) }
                        }
                    }
                    $crate::speck::SpeckVersion::Speck48_72 => {
                        if Self::SUFFIX < 3 {
                            let b = Align(bytes.map(|b| [b[3], b[4], b[5], 0]));
                            pb = unsafe { $load(b.0.as_ptr().cast()) }
                        }
                        if Self::SUFFIX < 6 {
                            let c = Align(bytes.map(|b| [b[6], b[7], b[8], 0]));
                            pc = unsafe { $load(c.0.as_ptr().cast()) }
                        }
                    }
                    $crate::speck::SpeckVersion::Speck48_96 => {
                        if Self::SUFFIX < 3 {
                            let b = Align(bytes.map(|b| [b[3], b[4], b[5], 0]));
                            pb = unsafe { $load(b.0.as_ptr().cast()) }
                        }
                        if Self::SUFFIX < 6 {
                            let c = Align(bytes.map(|b| [b[6], b[7], b[8], 0]));
                            pc = unsafe { $load(c.0.as_ptr().cast()) }
                        }
                        if Self::SUFFIX < 9 {
                            let d = Align(bytes.map(|b| [b[9], b[10], b[11], 0]));
                            pd = unsafe { $load(d.0.as_ptr().cast()) }
                        }
                    }
                    $crate::speck::SpeckVersion::Speck64_96 => {
                        if Self::SUFFIX < 4 {
                            let b = Align(bytes.map(|b| [b[4], b[5], b[6], b[7]]));
                            pb = unsafe { $load(b.0.as_ptr().cast()) }
                        }
                        if Self::SUFFIX < 8 {
                            let c = Align(bytes.map(|b| [b[8], b[9], b[10], b[11]]));
                            pc = unsafe { $load(c.0.as_ptr().cast()) }
                        }
                    }
                    $crate::speck::SpeckVersion::Speck64_128 => {
                        if Self::SUFFIX < 4 {
                            let b = Align(bytes.map(|b| [b[4], b[5], b[6], b[7]]));
                            pb = unsafe { $load(b.0.as_ptr().cast()) }
                        }
                        if Self::SUFFIX < 8 {
                            let c = Align(bytes.map(|b| [b[8], b[9], b[10], b[11]]));
                            pc = unsafe { $load(c.0.as_ptr().cast()) }
                        }
                        if Self::SUFFIX < 12 {
                            let d = Align(bytes.map(|b| [b[12], b[13], b[14], b[15]]));
                            pd = unsafe { $load(d.0.as_ptr().cast()) }
                        }
                    }
                    $crate::speck::SpeckVersion::Speck96_96 => {
                        if Self::SUFFIX < 6 {
                            let b = Align(bytes.map(|b| [b[6], b[7], b[8], b[9], b[10], b[11], 0, 0]));
                            pb = unsafe { $load(b.0.as_ptr().cast()) }
                        }
                    }
                    $crate::speck::SpeckVersion::Speck96_144 => {
                        if Self::SUFFIX < 6 {
                            let b = Align(bytes.map(|b| [b[6], b[7], b[8], b[9], b[10], b[11], 0, 0]));
                            pb = unsafe { $load(b.0.as_ptr().cast()) }
                        }
                        if Self::SUFFIX < 12 {
                            let c = Align(bytes.map(|b| [b[12], b[13], b[14], b[15], b[16], b[17], 0, 0]));
                            pc = unsafe { $load(c.0.as_ptr().cast()) }
                        }
                    }
                    $crate::speck::SpeckVersion::Speck128_128 => {
                        if Self::SUFFIX < 8 {
                            let b = Align(
                                bytes.map(|b| [b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]]),
                            );
                            pb = unsafe { $load(b.0.as_ptr().cast()) }
                        }
                    }
                    $crate::speck::SpeckVersion::Speck128_192 => {
                        if Self::SUFFIX < 8 {
                            let b = Align(
                                bytes.map(|b| [b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]]),
                            );
                            pb = unsafe { $load(b.0.as_ptr().cast()) }
                        }
                        if Self::SUFFIX < 16 {
                            let c = Align(
                                bytes.map(|b| [b[16], b[17], b[18], b[19], b[20], b[21], b[22], b[23]]),
                            );
                            pc = unsafe { $load(c.0.as_ptr().cast()) }
                        }
                    }
                    $crate::speck::SpeckVersion::Speck128_256 => {
                        if Self::SUFFIX < 8 {
                            let b = Align(
                                bytes.map(|b| [b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]]),
                            );
                            pb = unsafe { $load(b.0.as_ptr().cast()) }
                        }
                        if Self::SUFFIX < 16 {
                            let c = Align(
                                bytes.map(|b| [b[16], b[17], b[18], b[19], b[20], b[21], b[22], b[23]]),
                            );
                            pc = unsafe { $load(c.0.as_ptr().cast()) }
                        }
                        if Self::SUFFIX < 24 {
                            let d = Align(
                                bytes.map(|b| [b[24], b[25], b[26], b[27], b[28], b[29], b[30], b[31]]),
                            );
                            pd = unsafe { $load(d.0.as_ptr().cast()) }
                        }
                    }
                }

                Self { bytes, pb, pc, pd }
            }

            pub fn update(&mut self, v: [u64; LANES]) {
                for (i, v) in v.iter().enumerate().take(LANES) {
                    let suffix = v.to_le_bytes();
                    self.bytes[i][..Self::SUFFIX].copy_from_slice(&suffix[..Self::SUFFIX]);
                }
            }

            pub fn get(&self, i: usize) -> $crate::search::domain::key::Key<BYTES, PREFIX> {
                let row = &self.bytes[i];
                $crate::search::domain::key::Key::new_from_bytes(row)
            }

            pub fn as_bytes(&self) -> &[[u8; BYTES]; LANES] {
                &self.bytes
            }

            pub fn to_vec(&self) -> [Vec<u8>; LANES] {
                self.as_bytes().map(|b| b.to_vec())
            }
        }

        impl<const PREFIX: usize> $name<LANES_U16, 8, PREFIX> {
            #[cfg(target_arch = "x86_64")]
            #[target_feature(enable = $feature)]
            #[doc = "# Safety"]
            #[doc = concat!("Caller must ensure CPU support for `", $feature, "` before calling this function.")]
            pub fn u16x4_key(&self) -> [$vector; 4] {
                let a = Align(self.bytes.map(|b| [b[0], b[1]]));

                unsafe {
                    let ka = $load(a.0.as_ptr().cast());

                    let kb: $vector;
                    if Self::SUFFIX < 2 {
                        kb = self.pb;
                    } else {
                        let b = Align(self.bytes.map(|b| [b[2], b[3]]));
                        kb = $load(b.0.as_ptr().cast());
                    }

                    let kc: $vector;
                    if Self::SUFFIX < 4 {
                        kc = self.pc;
                    } else {
                        let c = Align(self.bytes.map(|b| [b[4], b[5]]));
                        kc = $load(c.0.as_ptr().cast());
                    }

                    let kd: $vector;
                    if Self::SUFFIX < 6 {
                        kd = self.pd;
                    } else {
                        let d = Align(self.bytes.map(|b| [b[6], b[7]]));
                        kd = $load(d.0.as_ptr().cast());
                    }

                    [ka, kb, kc, kd]
                }
            }
        }

        impl<const PREFIX: usize> $name<LANES_U32, 9, PREFIX> {
            #[cfg(target_arch = "x86_64")]
            #[target_feature(enable = $feature)]
            #[doc = "# Safety"]
            #[doc = concat!("Caller must ensure CPU support for `", $feature, "` before calling this function.")]
            pub fn u24x3_key(&self) -> [$vector; 3] {
                let a = Align(self.bytes.map(|b| [b[0], b[1], b[2], 0]));

                unsafe {
                    let ka = $load(a.0.as_ptr().cast());

                    let kb: $vector;
                    if Self::SUFFIX < 3 {
                        kb = self.pb;
                    } else {
                        let b = Align(self.bytes.map(|b| [b[3], b[4], b[5], 0]));
                        kb = $load(b.0.as_ptr().cast());
                    }

                    let kc: $vector;
                    if Self::SUFFIX < 6 {
                        kc = self.pc;
                    } else {
                        let c = Align(self.bytes.map(|b| [b[6], b[7], b[8], 0]));
                        kc = $load(c.0.as_ptr().cast());
                    }

                    [ka, kb, kc]
                }
            }
        }

        impl<const PREFIX: usize> $name<LANES_U32, 12, PREFIX> {
            #[cfg(target_arch = "x86_64")]
            #[target_feature(enable = $feature)]
            #[doc = "# Safety"]
            #[doc = concat!("Caller must ensure CPU support for `", $feature, "` before calling this function.")]
            pub fn u24x4_key(&self) -> [$vector; 4] {
                let a = Align(self.bytes.map(|b| [b[0], b[1], b[2], 0]));

                unsafe {
                    let ka = $load(a.0.as_ptr().cast());

                    let kb: $vector;
                    if Self::SUFFIX < 3 {
                        kb = self.pb;
                    } else {
                        let b = Align(self.bytes.map(|b| [b[3], b[4], b[5], 0]));
                        kb = $load(b.0.as_ptr().cast());
                    }

                    let kc: $vector;
                    if Self::SUFFIX < 6 {
                        kc = self.pc;
                    } else {
                        let c = Align(self.bytes.map(|b| [b[6], b[7], b[8], 0]));
                        kc = $load(c.0.as_ptr().cast());
                    }

                    let kd: $vector;
                    if Self::SUFFIX < 9 {
                        kd = self.pd;
                    } else {
                        let d = Align(self.bytes.map(|b| [b[9], b[10], b[11], 0]));
                        kd = $load(d.0.as_ptr().cast());
                    }

                    [ka, kb, kc, kd]
                }
            }

            #[cfg(target_arch = "x86_64")]
            #[target_feature(enable = $feature)]
            #[doc = "# Safety"]
            #[doc = concat!("Caller must ensure CPU support for `", $feature, "` before calling this function.")]
            pub fn u32x3_key(&self) -> [$vector; 3] {
                let a = Align(self.bytes.map(|b| [b[0], b[1], b[2], b[3]]));

                unsafe {
                    let ka = $load(a.0.as_ptr().cast());

                    let kb: $vector;
                    if Self::SUFFIX < 4 {
                        kb = self.pb;
                    } else {
                        let b = Align(self.bytes.map(|b| [b[4], b[5], b[6], b[7]]));
                        kb = $load(b.0.as_ptr().cast());
                    }

                    let kc: $vector;
                    if Self::SUFFIX < 8 {
                        kc = self.pc;
                    } else {
                        let c = Align(self.bytes.map(|b| [b[8], b[9], b[10], b[11]]));
                        kc = $load(c.0.as_ptr().cast());
                    }

                    [ka, kb, kc]
                }
            }
        }

        impl<const PREFIX: usize> $name<LANES_U32, 16, PREFIX> {
            #[cfg(target_arch = "x86_64")]
            #[target_feature(enable = $feature)]
            #[doc = "# Safety"]
            #[doc = concat!("Caller must ensure CPU support for `", $feature, "` before calling this function.")]
            pub fn u32x4_key(&self) -> [$vector; 4] {
                let a = Align(self.bytes.map(|b| [b[0], b[1], b[2], b[3]]));

                unsafe {
                    let ka = $load(a.0.as_ptr().cast());

                    let kb: $vector;
                    if Self::SUFFIX < 4 {
                        kb = self.pb;
                    } else {
                        let b = Align(self.bytes.map(|b| [b[4], b[5], b[6], b[7]]));
                        kb = $load(b.0.as_ptr().cast());
                    }

                    let kc: $vector;
                    if Self::SUFFIX < 8 {
                        kc = self.pc;
                    } else {
                        let c = Align(self.bytes.map(|b| [b[8], b[9], b[10], b[11]]));
                        kc = $load(c.0.as_ptr().cast());
                    }

                    let kd: $vector;
                    if Self::SUFFIX < 12 {
                        kd = self.pd;
                    } else {
                        let d = Align(self.bytes.map(|b| [b[12], b[13], b[14], b[15]]));
                        kd = $load(d.0.as_ptr().cast());
                    }

                    [ka, kb, kc, kd]
                }
            }
        }

        impl<const PREFIX: usize> $name<LANES_U64, 12, PREFIX> {
            #[cfg(target_arch = "x86_64")]
            #[target_feature(enable = $feature)]
            #[doc = "# Safety"]
            #[doc = concat!("Caller must ensure CPU support for `", $feature, "` before calling this function.")]
            pub fn u48x2_key(&self) -> [$vector; 2] {
                let a = Align(
                    self.bytes
                        .map(|b| [b[0], b[1], b[2], b[3], b[4], b[5], 0, 0]),
                );

                unsafe {
                    let ka = $load(a.0.as_ptr().cast());

                    let kb: $vector;
                    if Self::SUFFIX < 6 {
                        kb = self.pb;
                    } else {
                        let b = Align(
                            self.bytes
                                .map(|b| [b[6], b[7], b[8], b[9], b[10], b[11], 0, 0]),
                        );
                        kb = $load(b.0.as_ptr().cast());
                    }

                    [ka, kb]
                }
            }
        }

        impl<const PREFIX: usize> $name<LANES_U64, 18, PREFIX> {
            #[cfg(target_arch = "x86_64")]
            #[target_feature(enable = $feature)]
            #[doc = "# Safety"]
            #[doc = concat!("Caller must ensure CPU support for `", $feature, "` before calling this function.")]
            pub fn u48x3_key(&self) -> [$vector; 3] {
                let a = Align(
                    self.bytes
                        .map(|b| [b[0], b[1], b[2], b[3], b[4], b[5], 0, 0]),
                );

                unsafe {
                    let ka = $load(a.0.as_ptr().cast());

                    let kb: $vector;
                    if Self::SUFFIX < 6 {
                        kb = self.pb;
                    } else {
                        let b = Align(
                            self.bytes
                                .map(|b| [b[6], b[7], b[8], b[9], b[10], b[11], 0, 0]),
                        );
                        kb = $load(b.0.as_ptr().cast());
                    }

                    let kc: $vector;
                    if Self::SUFFIX < 12 {
                        kc = self.pc;
                    } else {
                        let c = Align(
                            self.bytes
                                .map(|b| [b[12], b[13], b[14], b[15], b[16], b[17], 0, 0]),
                        );
                        kc = $load(c.0.as_ptr().cast());
                    }

                    [ka, kb, kc]
                }
            }
        }

        impl<const PREFIX: usize> $name<LANES_U64, 16, PREFIX> {
            #[cfg(target_arch = "x86_64")]
            #[target_feature(enable = $feature)]
            #[doc = "# Safety"]
            #[doc = concat!("Caller must ensure CPU support for `", $feature, "` before calling this function.")]
            pub fn u64x2_key(&self) -> [$vector; 2] {
                let a = Align(
                    self.bytes
                        .map(|b| [b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]),
                );

                unsafe {
                    let ka = $load(a.0.as_ptr().cast());

                    let kb: $vector;
                    if Self::SUFFIX < 8 {
                        kb = self.pb;
                    } else {
                        let b = Align(
                            self.bytes
                                .map(|b| [b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]]),
                        );
                        kb = $load(b.0.as_ptr().cast());
                    }

                    [ka, kb]
                }
            }
        }

        impl<const PREFIX: usize> $name<LANES_U64, 24, PREFIX> {
            #[cfg(target_arch = "x86_64")]
            #[target_feature(enable = $feature)]
            #[doc = "# Safety"]
            #[doc = concat!("Caller must ensure CPU support for `", $feature, "` before calling this function.")]
            pub fn u64x3_key(&self) -> [$vector; 3] {
                let a = Align(
                    self.bytes
                        .map(|b| [b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]),
                );

                unsafe {
                    let ka = $load(a.0.as_ptr().cast());

                    let kb: $vector;
                    if Self::SUFFIX < 4 {
                        kb = self.pb;
                    } else {
                        let b = Align(
                            self.bytes
                                .map(|b| [b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]]),
                        );
                        kb = $load(b.0.as_ptr().cast());
                    }

                    let kc: $vector;
                    if Self::SUFFIX < 8 {
                        kc = self.pc;
                    } else {
                        let c = Align(
                            self.bytes
                                .map(|b| [b[16], b[17], b[18], b[19], b[20], b[21], b[22], b[23]]),
                        );
                        kc = $load(c.0.as_ptr().cast());
                    }

                    [ka, kb, kc]
                }
            }
        }

        impl<const PREFIX: usize> $name<LANES_U64, 32, PREFIX> {
            #[cfg(target_arch = "x86_64")]
            #[target_feature(enable = $feature)]
            #[doc = "# Safety"]
            #[doc = concat!("Caller must ensure CPU support for `", $feature, "` before calling this function.")]
            pub fn u64x4_key(&self) -> [$vector; 4] {
                let a = Align(
                    self.bytes
                        .map(|b| [b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]),
                );

                unsafe {
                    let ka = $load(a.0.as_ptr().cast());

                    let kb: $vector;
                    if Self::SUFFIX < 4 {
                        kb = self.pb;
                    } else {
                        let b = Align(
                            self.bytes
                                .map(|b| [b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]]),
                        );
                        kb = $load(b.0.as_ptr().cast());
                    }

                    let kc: $vector;
                    if Self::SUFFIX < 8 {
                        kc = self.pc;
                    } else {
                        let c = Align(
                            self.bytes
                                .map(|b| [b[16], b[17], b[18], b[19], b[20], b[21], b[22], b[23]]),
                        );
                        kc = $load(c.0.as_ptr().cast());
                    }

                    let kd: $vector;
                    if Self::SUFFIX < 12 {
                        kd = self.pd;
                    } else {
                        let d = Align(
                            self.bytes
                                .map(|b| [b[24], b[25], b[26], b[27], b[28], b[29], b[30], b[31]]),
                        );
                        kd = $load(d.0.as_ptr().cast());
                    }

                    [ka, kb, kc, kd]
                }
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;
            use rstest::rstest;

            #[rstest]
            #[case([], 0x0100_0908_1110_1918u64, [[0x1918u16; LANES_U16], [0x1110u16; LANES_U16], [0x0908u16; LANES_U16], [0x0100u16; LANES_U16]]
            )]
            fn key_conversion_32_64(
                #[case] prefix: [u8; 0],
                #[case] value: u64,
                #[case] expected: [[u16; LANES_U16]; 4],
            ) {
                if !is_x86_feature_detected!($feature) {
                    return;
                }
                unsafe {
                    let key = $name::<LANES_U16, 8, 0>::new(
                        &prefix,
                        [value; LANES_U16],
                        $crate::speck::SpeckVersion::Speck32_64,
                    );
                    let w: [[u16; LANES_U16]; 4] = key.u16x4_key().map(|r| std::mem::transmute(r));
                    assert_eq!(w[0], expected[0]);
                    assert_eq!(w[1], expected[1]);
                    assert_eq!(w[2], expected[2]);
                    assert_eq!(w[3], expected[3]);
                }
            }

            #[rstest]
            #[case([0x02], 0x0100_0a09_0812_1110u64, [[0x0012_1110u32; LANES_U32], [0x000a_0908u32; LANES_U32], [0x0002_0100u32; LANES_U32]]
            )]
            fn key_conversion_48_72(
                #[case] prefix: [u8; 1],
                #[case] value: u64,
                #[case] expected: [[u32; LANES_U32]; 3],
            ) {
                if !is_x86_feature_detected!($feature) {
                    return;
                }
                unsafe {
                    let key = $name::<LANES_U32, 9, 1>::new(
                        &prefix,
                        [value; LANES_U32],
                        $crate::speck::SpeckVersion::Speck48_72,
                    );
                    let w: [[u32; LANES_U32]; 3] = key.u24x3_key().map(|r| std::mem::transmute(r));
                    assert_eq!(w[0], expected[0]);
                    assert_eq!(w[1], expected[1]);
                    assert_eq!(w[2], expected[2]);
                }
            }

            #[rstest]
            #[case([0x0a, 0x00, 0x01, 0x02], 0x0908_1211_101a_1918u64, [[0x001a_1918u32; LANES_U32], [0x0012_1110u32; LANES_U32], [0x000a_0908u32; LANES_U32], [0x0002_0100u32; LANES_U32]]
            )]
            fn key_conversion_48_96(
                #[case] prefix: [u8; 4],
                #[case] value: u64,
                #[case] expected: [[u32; LANES_U32]; 4],
            ) {
                if !is_x86_feature_detected!($feature) {
                    return;
                }
                unsafe {
                    let key = $name::<LANES_U32, 12, 4>::new(
                        &prefix,
                        [value; LANES_U32],
                        $crate::speck::SpeckVersion::Speck48_96,
                    );
                    let w: [[u32; LANES_U32]; 4] = key.u24x4_key().map(|r| std::mem::transmute(r));
                    assert_eq!(w[0], expected[0]);
                    assert_eq!(w[1], expected[1]);
                    assert_eq!(w[2], expected[2]);
                    assert_eq!(w[3], expected[3]);
                }
            }

            #[rstest]
            #[case([0x00, 0x01, 0x02, 0x03], 0x0b0a_0908_1312_1110u64, [[0x1312_1110u32; LANES_U32], [0x0b0a_0908u32; LANES_U32], [0x0302_0100u32; LANES_U32]]
            )]
            fn key_conversion_64_96(
                #[case] prefix: [u8; 4],
                #[case] value: u64,
                #[case] expected: [[u32; LANES_U32]; 3],
            ) {
                if !is_x86_feature_detected!($feature) {
                    return;
                }
                unsafe {
                    let key = $name::<LANES_U32, 12, 4>::new(
                        &prefix,
                        [value; LANES_U32],
                        $crate::speck::SpeckVersion::Speck64_96,
                    );
                    let w: [[u32; LANES_U32]; 3] = key.u32x3_key().map(|r| std::mem::transmute(r));
                    assert_eq!(w[0], expected[0]);
                    assert_eq!(w[1], expected[1]);
                    assert_eq!(w[2], expected[2]);
                }
            }

            #[rstest]
            #[case([0x08, 0x09, 0x0a, 0x0b, 0x00, 0x01, 0x02, 0x03], 0x1312_1110_1b1a_1918u64, [[0x1b1a_1918u32; LANES_U32], [0x1312_1110u32; LANES_U32], [0x0b0a_0908u32; LANES_U32], [0x0302_0100u32; LANES_U32]]
            )]
            fn key_conversion_64_128(
                #[case] prefix: [u8; 8],
                #[case] value: u64,
                #[case] expected: [[u32; LANES_U32]; 4],
            ) {
                if !is_x86_feature_detected!($feature) {
                    return;
                }
                unsafe {
                    let key = $name::<LANES_U32, 16, 8>::new(
                        &prefix,
                        [value; LANES_U32],
                        $crate::speck::SpeckVersion::Speck64_128,
                    );
                    let w: [[u32; LANES_U32]; 4] = key.u32x4_key().map(|r| std::mem::transmute(r));
                    assert_eq!(w[0], expected[0]);
                    assert_eq!(w[1], expected[1]);
                    assert_eq!(w[2], expected[2]);
                    assert_eq!(w[3], expected[3]);
                }
            }

            #[rstest]
            #[case([0x02, 0x03, 0x04, 0x05], 0x0100_0d0c_0b0a_0908u64, [[0x0000_0d0c_0b0a_0908u64; LANES_U64], [0x0000_0504_0302_0100u64; LANES_U64]]
            )]
            fn key_conversion_96_96(
                #[case] prefix: [u8; 4],
                #[case] value: u64,
                #[case] expected: [[u64; LANES_U64]; 2],
            ) {
                if !is_x86_feature_detected!($feature) {
                    return;
                }
                unsafe {
                    let key = $name::<LANES_U64, 12, 4>::new(
                        &prefix,
                        [value; LANES_U64],
                        $crate::speck::SpeckVersion::Speck96_96,
                    );
                    let w: [[u64; LANES_U64]; 2] = key.u48x2_key().map(|r| std::mem::transmute(r));
                    assert_eq!(w[0], expected[0]);
                    assert_eq!(w[1], expected[1]);
                }
            }

            #[rstest]
            #[case(
                [0x0a, 0x0b, 0x0c, 0x0d, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05],
                0x0908_1514_1312_1110u64,
                [[0x0000_1514_1312_1110u64; LANES_U64], [0x0000_0d0c_0b0a_0908u64; LANES_U64], [0x0000_0504_0302_0100u64; LANES_U64]],
            )]
            fn key_conversion_96_144(
                #[case] prefix: [u8; 10],
                #[case] value: u64,
                #[case] expected: [[u64; LANES_U64]; 3],
            ) {
                if !is_x86_feature_detected!($feature) {
                    return;
                }
                unsafe {
                    let key = $name::<LANES_U64, 18, 10>::new(
                        &prefix,
                        [value; LANES_U64],
                        $crate::speck::SpeckVersion::Speck96_144,
                    );
                    let w: [[u64; LANES_U64]; 3] = key.u48x3_key().map(|r| std::mem::transmute(r));
                    assert_eq!(w[0], expected[0]);
                    assert_eq!(w[1], expected[1]);
                    assert_eq!(w[2], expected[2]);
                }
            }

            #[rstest]
            #[case(
                [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07],
                0x0f0e_0d0c_0b0a_0908u64,
                [[0x0f0e_0d0c_0b0a_0908u64; LANES_U64], [0x0706_0504_0302_0100u64; LANES_U64]],
            )]
            fn key_conversion_128_128(
                #[case] prefix: [u8; 8],
                #[case] value: u64,
                #[case] expected: [[u64; LANES_U64]; 2],
            ) {
                if !is_x86_feature_detected!($feature) {
                    return;
                }
                unsafe {
                    let key = $name::<LANES_U64, 16, 8>::new(
                        &prefix,
                        [value; LANES_U64],
                        $crate::speck::SpeckVersion::Speck128_128,
                    );
                    let w: [[u64; LANES_U64]; 2] = key.u64x2_key().map(|r| std::mem::transmute(r));
                    assert_eq!(w[0], expected[0]);
                    assert_eq!(w[1], expected[1]);
                }
            }

            #[rstest]
            #[case(
                [0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07],
                0x1716_1514_1312_1110u64,
                [[0x1716_1514_1312_1110u64; LANES_U64], [0x0f0e_0d0c_0b0a_0908u64; LANES_U64], [0x0706_0504_0302_0100u64; LANES_U64]],
            )]
            fn key_conversion_128_192(
                #[case] prefix: [u8; 16],
                #[case] value: u64,
                #[case] expected: [[u64; LANES_U64]; 3],
            ) {
                if !is_x86_feature_detected!($feature) {
                    return;
                }
                unsafe {
                    let key = $name::<LANES_U64, 24, 16>::new(
                        &prefix,
                        [value; LANES_U64],
                        $crate::speck::SpeckVersion::Speck128_192,
                    );
                    let w: [[u64; LANES_U64]; 3] = key.u64x3_key().map(|r| std::mem::transmute(r));
                    assert_eq!(w[0], expected[0]);
                    assert_eq!(w[1], expected[1]);
                    assert_eq!(w[2], expected[2]);
                }
            }

            #[rstest]
            #[case(
                [0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07],
                0x1f1e_1d1c_1b1a_1918u64,
                [[0x1f1e_1d1c_1b1a_1918u64; LANES_U64], [0x1716_1514_1312_1110u64; LANES_U64], [0x0f0e_0d0c_0b0a_0908u64; LANES_U64], [0x0706_0504_0302_0100u64; LANES_U64]],
            )]
            fn key_conversion_128_256(
                #[case] prefix: [u8; 24],
                #[case] value: u64,
                #[case] expected: [[u64; LANES_U64]; 4],
            ) {
                if !is_x86_feature_detected!($feature) {
                    return;
                }
                unsafe {
                    let key = $name::<LANES_U64, 32, 24>::new(
                        &prefix,
                        [value; LANES_U64],
                        $crate::speck::SpeckVersion::Speck128_256,
                    );
                    let w: [[u64; LANES_U64]; 4] = key.u64x4_key().map(|r| std::mem::transmute(r));
                    assert_eq!(w[0], expected[0]);
                    assert_eq!(w[1], expected[1]);
                    assert_eq!(w[2], expected[2]);
                    assert_eq!(w[3], expected[3]);
                }
            }
        }
    };
}

pub(crate) use define_backend_dispatch;
pub(crate) use define_search;
pub(crate) use impl_x86_simd_key;
