#[macro_export]
macro_rules! pairwise_operation_generic {
    ($lhs:ty, $rhs:ty, $output_type: ident, $trait:ident, $fn_name:ident, $op:tt) => {
        impl $trait<$rhs> for $lhs {
            type Output = $output_type;

            #[inline(always)]
            fn $fn_name(self, rhs: $rhs) -> Self::Output {
                vec3!(
                    self[0] $op rhs[0],
                    self[1] $op rhs[1],
                    self[2] $op rhs[2],
                )
            }
        }
    };
}

#[macro_export]
macro_rules! pairwise_operation {
    ($trait:ident, $fn_name:ident, $op:tt) => {
        pairwise_operation_generic!(Vec3, Vec3, Vec3, $trait, $fn_name, $op);
        pairwise_operation_generic!(Vec3, &Vec3, Vec3, $trait, $fn_name, $op);
        pairwise_operation_generic!(&Vec3, Vec3, Vec3, $trait, $fn_name, $op);
        pairwise_operation_generic!(&Vec3, &Vec3, Vec3, $trait, $fn_name, $op);
    };
}

#[macro_export]
macro_rules! operation_generic {
    ($lhs:ty, $rhs:ty, $output_type: ident, $trait:ident, $fn_name:ident, $op:tt) => {
        impl $trait<$rhs> for $lhs {
            type Output = $output_type;

            #[inline(always)]
            fn $fn_name(self, rhs: $rhs) -> Self::Output {
                vec3!(
                    self[0] $op rhs,
                    self[1] $op rhs,
                    self[2] $op rhs,
                )
            }
        }
    };
}

#[macro_export]
macro_rules! operation_generic_flip {
    ($lhs:ty, $rhs:ty, $output_type: ident, $trait:ident, $fn_name:ident, $op:tt) => {
        impl $trait<$rhs> for $lhs {
            type Output = $output_type;

            #[inline(always)]
            fn $fn_name(self, rhs: $rhs) -> Self::Output {
                vec3!(
                    self $op rhs[0],
                    self $op rhs[1],
                    self $op rhs[2],
                )
            }
        }
    };
}

#[macro_export]
macro_rules! floating_point_operation {
    ($trait:ident, $fn_name:ident, $op:tt) => {
        operation_generic!(Vec3, f64, Vec3, $trait, $fn_name, $op);
        operation_generic!(Vec3, &f64, Vec3, $trait, $fn_name, $op);
        operation_generic!(&Vec3, f64, Vec3, $trait, $fn_name, $op);
        operation_generic!(&Vec3, &f64, Vec3, $trait, $fn_name, $op);

        operation_generic_flip!(f64, Vec3, Vec3, $trait, $fn_name, $op);
        operation_generic_flip!(f64, &Vec3, Vec3, $trait, $fn_name, $op);
        operation_generic_flip!(&f64, Vec3, Vec3, $trait, $fn_name, $op);
        operation_generic_flip!(&f64, &Vec3, Vec3, $trait, $fn_name, $op);
    };
}

#[macro_export]
macro_rules! fn_on_each {
    ($fn_name:ident $(,)? $($args:ident),*) => {
        #[inline(always)]
        fn $fn_name(self, $($args:f64),*) -> Self {
            Self::new(self[0].$fn_name($($args),*), self[1].$fn_name($($args),*), self[2].$fn_name($($args),*))
        }
    };
    (& $fn_name:ident $(,)? $($args:ident),*) => {
        #[inline(always)]
        fn $fn_name(&self, $($args:f64),*) -> Self {
            Self::new(self[0].$fn_name($($args),*), self[1].$fn_name($($args),*), self[2].$fn_name($($args),*))
        }
    };
}

#[macro_export]
macro_rules! on_each_operation {
    ($trait:ident, $fn_name:ident) => {
        impl $trait for Vec3 {
            type Output = Self;

            fn_on_each!($fn_name);
        }
    };
}

#[macro_export]
macro_rules! pairwise_mut_operation_generic {
    ($lhs:ty, $rhs:ty, $trait:ident, $fn_name:ident, $op:tt) => {
        impl $trait<$rhs> for $lhs {

            #[inline(always)]
            fn $fn_name(&mut self, rhs: $rhs) {
                self[0] $op rhs[0];
                self[1] $op rhs[1];
                self[2] $op rhs[2];
            }
        }
    };
}

#[macro_export]
macro_rules! pairwise_mut_operation {
    ($trait:ident, $fn_name:ident, $op:tt) => {
        pairwise_mut_operation_generic!(Vec3, Vec3, $trait, $fn_name, $op);
        pairwise_mut_operation_generic!(Vec3, &Vec3, $trait, $fn_name, $op);
    };
}

#[macro_export]
macro_rules! vec3 {
    ($l: expr$(,)*) => {{
        Vec3([$l as f64, $l as f64, $l as f64])
    }};
    ($l0: expr, $l1: expr, $l2: expr$(,)*) => {{
        Vec3([$l0 as f64, $l1 as f64, $l2 as f64])
    }};
}
