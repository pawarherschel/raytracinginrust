#[macro_export]
macro_rules! point3 {
    ($l: expr) => {{
        Point3 {
            0: [$l as f64, $l as f64, $l as f64],
        }
    }};
    ($l0: expr, $l1: expr, $l2: expr) => {{
        Point3 {
            0: [$l0 as f64, $l1 as f64, $l2 as f64],
        }
    }};
}

#[macro_export]
macro_rules! x {
    ($x: expr) => {{
        $crate::point3![1_f64 * $x as f64, 0, 0]
    }};
    () => {{
        x!(1)
    }};
}

#[macro_export]
macro_rules! y {
    ($y: expr) => {{
        $crate::point3![0, 1_f64 * $y as f64, 0]
    }};
    () => {{
        y!(1)
    }};
}

#[macro_export]
macro_rules! z {
    ($z: expr) => {{
        $crate::point3![0, 0, -1_f64 * $z as f64]
    }};
    () => {{
        z!(1)
    }};
}
