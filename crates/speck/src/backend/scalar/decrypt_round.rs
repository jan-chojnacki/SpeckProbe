#[macro_export]
macro_rules! decrypt_round_inline {
    ($x:expr, $y:expr, $k:expr, $alpha:literal, $beta:literal) => {
        $y = $y.bitxor($x).rotate_right($beta);
        $x = $x.bitxor($k).wrapping_sub($y).rotate_left($alpha);
    };
}
