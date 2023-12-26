use crate::prelude::*;
use rstest::rstest;

#[macro_export]
macro_rules! lerp {
    ($start: expr, $t: expr, $end: expr) => {{
        (1_f64 - $t) * $start + $t * $end
    }};
}

#[macro_export]
macro_rules! remap {
    (value: $value: expr, from: $min1: expr, $max1: expr, to: $min2: expr, $max2: expr) => {{
        let value = $value;
        let min1 = $min1;
        let max1 = $max1;
        let min2 = $min2;
        let max2 = $max2;

        min2 + &(&(value - min1) * (max2 - min2) / (max1 - min1))
    }};
}

#[macro_export]
macro_rules! value {
    ($value: expr, between: $min: expr, and $max: expr) => {{
        $min < $value && $value < $max
    }};
    ($value: expr, not between: $min: expr, and $max: expr) => {{
        $value < $min || $max < $value
    }};
}
