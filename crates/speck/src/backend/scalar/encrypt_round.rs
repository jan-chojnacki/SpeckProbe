#[macro_export]
macro_rules! encrypt_round_inline {
    ($x:expr, $y:expr, $k:expr, $alpha:literal, $beta:literal) => {
        $x = $x.rotate_right($alpha).wrapping_add($y).bitxor($k);
        $y = $y.rotate_left($beta).bitxor($x);
    };
}
