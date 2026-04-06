use crate::backend::macors::define_runtime_variants_default;

define_runtime_variants_default!(scalar_32_64, 32_64, u16, bytes = 8);
define_runtime_variants_default!(scalar_48_72, 48_72, u32, bytes = 9);
define_runtime_variants_default!(scalar_48_96, 48_96, u32, bytes = 12);
define_runtime_variants_default!(scalar_64_96, 64_96, u32, bytes = 12);
define_runtime_variants_default!(scalar_64_128, 64_128, u32, bytes = 16);
define_runtime_variants_default!(scalar_96_96, 96_96, u64, bytes = 12);
define_runtime_variants_default!(scalar_96_144, 96_144, u64, bytes = 18);
define_runtime_variants_default!(scalar_128_128, 128_128, u64, bytes = 16);
define_runtime_variants_default!(scalar_128_192, 128_192, u64, bytes = 24);
define_runtime_variants_default!(scalar_128_256, 128_256, u64, bytes = 32);
