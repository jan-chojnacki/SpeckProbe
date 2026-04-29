use crate::runtime::backend::macros::define_backend_dispatch;

define_backend_dispatch! {
    simd = scalar_,
    versions = [
        (32_64,   bytes=8,  ew=u16, vw=u16, converter=|x| x),
        (48_72,   bytes=9,  ew=u32, vw=u32, converter=|x| x),
        (48_96,   bytes=12, ew=u32, vw=u32, converter=|x| x),
        (64_96,   bytes=12, ew=u32, vw=u32, converter=|x| x),
        (64_128,  bytes=16, ew=u32, vw=u32, converter=|x| x),
        (96_96,   bytes=12, ew=u64, vw=u64, converter=|x| x),
        (96_144,  bytes=18, ew=u64, vw=u64, converter=|x| x),
        (128_128, bytes=16, ew=u64, vw=u64, converter=|x| x),
        (128_192, bytes=24, ew=u64, vw=u64, converter=|x| x),
        (128_256, bytes=32, ew=u64, vw=u64, converter=|x| x),
    ]
}
