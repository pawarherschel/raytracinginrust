#[macro_export]
macro_rules! color {
    ($l: expr) => {{
        Color {
            0: [$l as f64, $l as f64, $l as f64],
        }
    }};
    ($l0: expr, $l1: expr, $l2: expr) => {{
        Color {
            0: [$l0 as f64, $l1 as f64, $l2 as f64],
        }
    }};
}

#[macro_export]
macro_rules! red {
    ($red: expr) => {{
        color![1, 0, 0] * $red as f64
    }};
    () => {{
        red!(1)
    }};
}

#[macro_export]
macro_rules! green {
    ($green: expr) => {{
        color![0, 1, 0] * $green as f64
    }};
    () => {{
        green!(1)
    }};
}

#[macro_export]
macro_rules! blue {
    ($blue: expr) => {{
        color![0, 0, 1] * $blue as f64
    }};
    () => {{
        blue!(1)
    }};
}

#[macro_export]
macro_rules! white {
    () => {{
        color!(1)
    }};
}
