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

        min2 + (value - min1) * (max2 - min2) / (max1 - min1)
    }};
}

// #[inline(always)]
// pub fn rand21(x: f64, y: f64) -> f64 {
//     let seed = vec3![12.9898, 78.233, 1243.1254];
//     let time = SystemTime::now()
//         .duration_since(SystemTime::UNIX_EPOCH)
//         .unwrap()
//         .as_secs_f64();
//     let cord = vec3![x, y, time];
//     (cord.dot(&seed).sin() * 43758.5453).fract()
// }

#[cfg(test)]
mod map_tests {
    use crate::color;
    use crate::prelude::Color;

    #[test]
    fn test0() {
        let value = 0_f64;
        let min1 = -1_f64;
        let max1 = 1_f64;
        let min2 = 0_f64;
        let max2 = 1_f64;

        let expected = 0.5;
        let actual = remap!(value: value, from: min1, max1, to: min2, max2);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test1() {
        let value = 0.5_f64;
        let min1 = -1_f64;
        let max1 = 1_f64;
        let min2 = 0_f64;
        let max2 = 1_f64;

        let expected = 0.75;
        let actual = remap!(value: value, from: min1, max1, to: min2, max2);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test2() {
        let value: Color = color!(0);
        let min1 = -1_f64;
        let max1 = 1_f64;
        let min2 = 0_f64;
        let max2 = 1_f64;

        let expected = color!(0.5);
        let actual = remap!(value: value, from: min1, max1, to: min2, max2);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test3() {
        let value: Color = color!(0.5);
        let min1 = -1_f64;
        let max1 = 1_f64;
        let min2 = 0_f64;
        let max2 = 1_f64;

        let expected = color!(0.75);
        let actual = remap!(value: value, from: min1, max1, to: min2, max2);

        assert_eq!(expected, actual);
    }
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
