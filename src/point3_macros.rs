#[macro_export]
macro_rules! point3 {
    ($l: expr) => {{
        Point3 {
            0: [$l as f32, $l as f32, $l as f32],
        }
    }};
    ($l0: expr, $l1: expr, $l2: expr) => {{
        Point3 {
            0: [$l0 as f32, $l1 as f32, $l2 as f32],
        }
    }};
}

#[macro_export]
macro_rules! x {
    ($x: expr) => {{
        $crate::point3![1_f32 * $x as f32, 0, 0]
    }};
    () => {{
        x!(1)
    }};
}

#[macro_export]
macro_rules! y {
    ($y: expr) => {{
        $crate::point3![0, 1_f32 * $y as f32, 0]
    }};
    () => {{
        y!(1)
    }};
}

#[macro_export]
macro_rules! z {
    ($z: expr) => {{
        $crate::point3![0, 0, -1_f32 * $z as f32]
    }};
    () => {{
        z!(1)
    }};
}
