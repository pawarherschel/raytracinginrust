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
        $crate::color![$red, 0, 0]
    }};
    () => {{
        $crate::red!(1)
    }};
}

#[macro_export]
macro_rules! green {
    ($green: expr) => {{
        $crate::color![0, $green, 0]
    }};
    () => {{
        $crate::green!(1)
    }};
}

#[macro_export]
macro_rules! blue {
    ($blue: expr) => {{
        $crate::color![0, 0, $blue]
    }};
    () => {{
        $crate::blue!(1)
    }};
}

#[macro_export]
macro_rules! white {
    () => {{
        $crate::color!(1)
    }};
}
