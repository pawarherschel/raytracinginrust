#[macro_export]
macro_rules! lerp {
    ($start: expr, $t: expr, $end: expr) => {{
        (1_f64 - $t) * $start + $t * $end
    }};
}
