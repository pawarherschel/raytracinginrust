#[macro_export]
macro_rules! vec3 {
    ($l: expr) => {{
        vec3!($l, $l, $l)
    }};
    ($l0: expr, $l1: expr, $l2: expr) => {{
        Vec3([$l0 as f64, $l1 as f64, $l2 as f64])
    }};
}
