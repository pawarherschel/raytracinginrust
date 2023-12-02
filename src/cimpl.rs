#[macro_export]
macro_rules! cimpl {
    //
    (trait: $trait:ident, self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3, 3) => {
        impl $trait<$rhs> for $lhs {
            type Output = $output;

            #[inline(always)]
            fn $fn(self, rhs: $rhs) -> Self::Output {
                Self::Output::new(
                    self[0] $op rhs[0],
                    self[1] $op rhs[1],
                    self[2] $op rhs[2],
                )
            }
        }
    };
    (trait: $trait:ident, ref self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3, 3) => {
        impl $trait<$rhs> for $lhs {
            type Output = $output;

            #[inline(always)]
            fn $fn(&self, rhs: $rhs) -> Self::Output {
                Self::Output::new(
                    self[0] $op rhs[0],
                    self[1] $op rhs[1],
                    self[2] $op rhs[2],
                )
            }
        }
    };
    (trait: $trait:ident, ref mut self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3, 3) => {
        impl $trait<$rhs> for $lhs {
            type Output = $output;

            #[inline(always)]
            fn $fn(&mut self, rhs: $rhs) -> Self::Output {
                Self::Output::new(
                    self[0] $op rhs[0],
                    self[1] $op rhs[1],
                    self[2] $op rhs[2],
                )
            }
        }
    };
    //
    (trait: $trait:ident, self: $lhs:ty, other: $rhs:ty, fn: $fn:ident, op: $op:tt | 3, 3) => {
        impl $trait<$rhs> for $lhs {
            #[inline(always)]
            fn $fn(self, rhs: $rhs) {unimplemented!()}
        }
    };
    (trait: $trait:ident, ref self: $lhs:ty, other: $rhs:ty, fn: $fn:ident, op: $op:tt | 3, 3) => {
        impl $trait<$rhs> for $lhs {
            #[inline(always)]
            fn $fn(&self, rhs: $rhs) {unimplemented!()}
        }
    };
    (trait: $trait:ident, ref mut self: $lhs:ty, other: $rhs:ty, fn: $fn:ident, op: $op:tt | 3, 3) => {
        impl $trait<$rhs> for $lhs {
            #[inline(always)]
            fn $fn(&mut self, rhs: $rhs) {
                self[0] $op rhs[0];
                self[1] $op rhs[1];
                self[2] $op rhs[2];
            }
        }
    };
    //
    (trait: $trait:ident, self: $lhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $trait for $lhs {
            type Output = $output;

            #[inline(always)]
            fn $fn(self) -> Self::Output {
                Self::Output::new(
                    $op self[0],
                    $op self[1],
                    $op self[2],
                )
            }
        }
    };
    (trait: $trait:ident, ref self: $lhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $trait for $lhs {
            type Output = $output;

            #[inline(always)]
            fn $fn(&self) -> Self::Output {unimplemented!()}
        }
    };
    (trait: $trait:ident, ref mut self: $lhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $trait for $lhs {
            type Output = $output;
            #[inline(always)]
            fn $fn(&mut self) -> Self::Output {unimplemented!()}
        }
    };
    //
    (trait: $trait:ident, self: $lhs:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $trait for $lhs {
            #[inline(always)]
            fn $fn(self) {unimplemented!()}
        }
    };
    (trait: $trait:ident, ref self: $lhs:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $trait for $lhs {
            #[inline(always)]
            fn $fn(&self) {unimplemented!()}
        }
    };
    (trait: $trait:ident, ref mut self: $lhs:ty, fn: $fn:ident, op: $op:tt | 3) => {

        impl $trait for $lhs {
            #[inline(always)]
            fn $fn(&mut self) {unimplemented!()}
        }
    };
    // reflect
    (self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3, 3) => {
        impl $lhs {
            #[inline(always)]
            pub fn $fn(self, rhs: $rhs) -> $output {unimplemented!()}
        }
    };
    (ref self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3, 3) => {
        impl $lhs {
            #[inline(always)]
            pub fn $fn(&self, rhs: $rhs) -> $output {unimplemented!()}
        }
    };
    (ref mut self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3, 3) => {
        impl $lhs {
            #[inline(always)]
            pub fn $fn(&mut self, rhs: $rhs) -> $output {unimplemented!()}
        }
    };
    //
    (self: $lhs:ty, other: $rhs:ty, fn: $fn:ident, op: $op:tt | 3, 3) => {
        impl $lhs {
            #[inline(always)]
            pub fn $fn(self, rhs: $rhs) {unimplemented!()}
        }
    };
    (ref self: $lhs:ty, other: $rhs:ty, fn: $fn:ident, op: $op:tt | 3, 3) => {
        impl $lhs {
            #[inline(always)]
            pub fn $fn(&self, rhs: $rhs) {unimplemented!()}
        }
    };
    (ref mut self: $lhs:ty, other: $rhs:ty, fn: $fn:ident, op: $op:tt | 3, 3) => {
        impl $lhs {
            #[inline(always)]
            pub fn $fn(&mut self, rhs: $rhs) {unimplemented!()}
        }
    };
    //
    (self: $lhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $lhs {
            #[inline(always)]
            pub fn $fn(self) -> $output {unimplemented!()}
        }
    };
    (ref self: $lhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $lhs {
            #[inline(always)]
            pub fn $fn(&self) -> $output {unimplemented!()}
        }
    };
    (ref mut self: $lhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $lhs {
            #[inline(always)]
            pub fn $fn(&mut self) -> $output {unimplemented!()}
        }
    };
    //
    (self: $lhs:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $lhs {
            #[inline(always)]
            pub fn $fn(self) {unimplemented!()}
        }
    };
    (ref self: $lhs:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $lhs {
            #[inline(always)]
            pub fn $fn(&self) {unimplemented!()}
        }
    };
    (ref mut self: $lhs:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $lhs {
            #[inline(always)]
            pub fn $fn(&mut self) {unimplemented!()}
        }
    };
    //
    (trait: $trait:ident, self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3, 1) => {
        impl $trait<$rhs> for $lhs {
            type Output = $output;

            #[inline(always)]
            fn $fn(self, rhs: $rhs) -> Self::Output {
                Self::Output::new(
                    self[0] $op rhs,
                    self[1] $op rhs,
                    self[2] $op rhs,
                )
            }
        }
    };
    (trait: $trait:ident, ref self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3, 1) => {
        impl $trait<$rhs> for $lhs {
            type Output = $output;

            #[inline(always)]
            fn $fn(&self, rhs: $rhs) -> Self::Output {unimplemented!()}
        }
    };
    (trait: $trait:ident, ref mut self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3, 1) => {
        impl $trait<$rhs> for $lhs {
            type Output = $output;

            #[inline(always)]
            fn $fn(&mut self, rhs: $rhs) -> Self::Output {unimplemented!()}
        }
    };
    //
    (trait: $trait:ident, self: $lhs:ty, other: $rhs:ty, fn: $fn:ident, op: $op:tt | 3, 1) => {
        impl $trait<$rhs> for $lhs {
            #[inline(always)]
            fn $fn(self, rhs: $rhs) {unimplemented!()}
        }
    };
    (trait: $trait:ident, ref self: $lhs:ty, other: $rhs:ty, fn: $fn:ident, op: $op:tt | 3, 1) => {
        impl $trait<$rhs> for $lhs {
            #[inline(always)]
            fn $fn(&self, rhs: $rhs) {unimplemented!()}
        }
    };
    (trait: $trait:ident, ref mut self: $lhs:ty, other: $rhs:ty, fn: $fn:ident, op: $op:tt | 3, 1) => {
        impl $trait<$rhs> for $lhs {
            #[inline(always)]
            fn $fn(&mut self, rhs: $rhs) {unimplemented!()}
        }
    };
    //
    (trait: $trait:ident, self: $lhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $trait for $lhs {
            type Output = $output;

            #[inline(always)]
            fn $fn(self) -> Self::Output {unimplemented!()}
        }
    };
    (trait: $trait:ident, ref self: $lhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $trait for $lhs {
            type Output = $output;

            #[inline(always)]
            fn $fn(&self) -> Self::Output {unimplemented!()}
        }
    };
    (trait: $trait:ident, ref mut self: $lhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $trait for $lhs {
            type Output = $output;
            #[inline(always)]
            fn $fn(&mut self) -> Self::Output {unimplemented!()}
        }
    };
    //
    (trait: $trait:ident, self: $lhs:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $trait for $lhs {
            #[inline(always)]
            fn $fn(self) {unimplemented!()}
        }
    };
    (trait: $trait:ident, ref self: $lhs:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $trait for $lhs {
            #[inline(always)]
            fn $fn(&self) {unimplemented!()}
        }
    };
    (trait: $trait:ident, ref mut self: $lhs:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $trait for $lhs {
            #[inline(always)]
            fn $fn(&mut self) {unimplemented!()}
        }
    };
    //
    (self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3, 1) => {
        impl $lhs {
            #[inline(always)]
            pub fn $fn(self, rhs: $rhs) -> $output {unimplemented!()}
        }
    };
    (ref self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3, 1) => {
        impl $lhs {
            #[inline(always)]
            pub fn $fn(&self, rhs: $rhs) -> $output {unimplemented!()}
        }
    };
    (ref mut self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3, 1) => {
        impl $lhs {
            #[inline(always)]
            pub fn $fn(&mut self, rhs: $rhs) -> $output {unimplemented!()}
        }
    };
    //
    (self: $lhs:ty, other: $rhs:ty, fn: $fn:ident, op: $op:tt | 3, 1) => {
        impl $lhs {
            #[inline(always)]
            pub fn $fn(self, rhs: $rhs) {unimplemented!()}
        }
    };
    (ref self: $lhs:ty, other: $rhs:ty, fn: $fn:ident, op: $op:tt | 3, 1) => {
        impl $lhs {
            #[inline(always)]
            pub fn $fn(&self, rhs: $rhs) {unimplemented!()}
        }
    };
    (ref mut self: $lhs:ty, other: $rhs:ty, fn: $fn:ident, op: $op:tt | 3, 1) => {
        impl $lhs {
            #[inline(always)]
            pub fn $fn(&mut self, rhs: $rhs) {unimplemented!()}
        }
    };
    //
    (self: $lhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $lhs {
            #[inline(always)]
            pub fn $fn(self) -> $output {unimplemented!()}
        }
    };
    (ref self: $lhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $lhs {
            #[inline(always)]
            pub fn $fn(&self) -> $output {unimplemented!()}
        }
    };
    (ref mut self: $lhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $lhs {
            #[inline(always)]
            pub fn $fn(&mut self) -> $output {unimplemented!()}
        }
    };
    //
    (self: $lhs:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $lhs {
            #[inline(always)]
            pub fn $fn(self) {unimplemented!()}
        }
    };
    (ref self: $lhs:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $lhs {
            #[inline(always)]
            pub fn $fn(&self) {unimplemented!()}
        }
    };
    (ref mut self: $lhs:ty, fn: $fn:ident, op: $op:tt | 3) => {
        impl $lhs {
            #[inline(always)]
            pub fn $fn(&mut self) {unimplemented!()}
        }
    };
    //
    (trait: $trait:ident, self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 1, 3) => {
        impl $trait<$rhs> for $lhs {
            type Output = $output;

            #[inline(always)]
            fn $fn(self, rhs: $rhs) -> Self::Output {
                Self::Output::new(self $op rhs[0], self $op rhs[1], self $op rhs[2])
            }
        }
    };
    (trait: $trait:ident, ref self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 1, 3) => {
        impl $trait<$rhs> for $lhs {
            type Output = $output;

            #[inline(always)]
            fn $fn(&self, rhs: $rhs) -> Self::Output {unimplemented!()}
        }
    };
    (trait: $trait:ident, ref mut self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 1, 3) => {
        impl $trait<$rhs> for $lhs {
            type Output = $output;

            #[inline(always)]
            fn $fn(&mut self, rhs: $rhs) -> Self::Output {unimplemented!()}
        }
    };
    //
    (trait: $trait:ident, self: $lhs:ty, other: $rhs:ty, fn: $fn:ident, op: $op:tt | 1, 3) => {
        impl $trait<$rhs> for $lhs {
            #[inline(always)]
            fn $fn(self, rhs: $rhs) {unimplemented!()}
        }
    };
    (trait: $trait:ident, ref self: $lhs:ty, other: $rhs:ty, fn: $fn:ident, op: $op:tt | 1, 3) => {
        impl $trait<$rhs> for $lhs {
            #[inline(always)]
            fn $fn(&self, rhs: $rhs) {unimplemented!()}
        }
    };
    (trait: $trait:ident, ref mut self: $lhs:ty, other: $rhs:ty, fn: $fn:ident, op: $op:tt | 1, 3) => {
        impl $trait<$rhs> for $lhs {
            #[inline(always)]
            fn $fn(&mut self, rhs: $rhs) {unimplemented!()}
        }
    };
    //
    (trait: $trait:ident, self: $lhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 1) => {
        impl $trait for $lhs {
            type Output = $output;

            #[inline(always)]
            fn $fn(self) -> Self::Output {unimplemented!()}
        }
    };
    (trait: $trait:ident, ref self: $lhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 1) => {
        impl $trait for $lhs {
            type Output = $output;

            #[inline(always)]
            fn $fn(&self) -> Self::Output {unimplemented!()}
        }
    };
    (trait: $trait:ident, ref mut self: $lhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 1) => {
        impl $trait for $lhs {
            type Output = $output;
            #[inline(always)]
            fn $fn(&mut self) -> Self::Output {unimplemented!()}
        }
    };
    //
    (trait: $trait:ident, self: $lhs:ty, fn: $fn:ident, op: $op:tt | 1) => {
        impl $trait for $lhs {
            #[inline(always)]
            fn $fn(self) {unimplemented!()}
        }
    };
    (trait: $trait:ident, ref self: $lhs:ty, fn: $fn:ident, op: $op:tt | 1) => {
        impl $trait for $lhs {
            #[inline(always)]
            fn $fn(&self) {unimplemented!()}
        }
    };
    (trait: $trait:ident, ref mut self: $lhs:ty, fn: $fn:ident, op: $op:tt | 1) => {
        impl $trait for $lhs {
            #[inline(always)]
            fn $fn(&mut self) {unimplemented!()}
        }
    };
    //
    (self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 1, 3) => {
        impl $lhs {
            #[inline(always)]
            pub fn $fn(self, rhs: $rhs) -> $output {unimplemented!()}
        }
    };
    (ref self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 1, 3) => {
        impl $lhs {
            #[inline(always)]
            pub fn $fn(&self, rhs: $rhs) -> $output {unimplemented!()}
        }
    };
    (ref mut self: $lhs:ty, other: $rhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 1, 3) => {
        impl $lhs {
            #[inline(always)]
            pub fn $fn(&mut self, rhs: $rhs) -> $output {unimplemented!()}
        }
    };
    //
    (self: $lhs:ty, other: $rhs:ty, fn: $fn:ident, op: $op:tt | 1, 3) => {
        impl $lhs {
            #[inline(always)]
            pub fn $fn(self, rhs: $rhs) {unimplemented!()}
        }
    };
    (ref self: $lhs:ty, other: $rhs:ty, fn: $fn:ident, op: $op:tt | 1, 3) => {
        impl $lhs {
            #[inline(always)]
            pub fn $fn(&self, rhs: $rhs) {unimplemented!()}
        }
    };
    (ref mut self: $lhs:ty, other: $rhs:ty, fn: $fn:ident, op: $op:tt | 1, 3) => {
        impl $lhs {
            #[inline(always)]
            pub fn $fn(&mut self, rhs: $rhs) {unimplemented!()}
        }
    };
    //
    (self: $lhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 1) => {
        impl $lhs {
            #[inline(always)]
            pub fn $fn(self) -> $output {unimplemented!()}
        }
    };
    (ref self: $lhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 1) => {
        impl $lhs {
            #[inline(always)]
            pub fn $fn(&self) -> $output {unimplemented!()}
        }
    };
    (ref mut self: $lhs:ty, output: $output:ty, fn: $fn:ident, op: $op:tt | 1) => {
        impl $lhs {
            #[inline(always)]
            pub fn $fn(&mut self) -> $output {unimplemented!()}
        }
    };
    //
    (self: $lhs:ty, fn: $fn:ident, op: $op:tt | 1) => {
        impl $lhs {
            #[inline(always)]
            pub fn $fn(self) {unimplemented!()}
        }
    };
    (ref self: $lhs:ty, fn: $fn:ident, op: $op:tt | 1) => {
        impl $lhs {
            #[inline(always)]
            pub fn $fn(&self) {unimplemented!()}
        }
    };
    (ref mut self: $lhs:ty, fn: $fn:ident, op: $op:tt | 1) => {
        impl $lhs {
            #[inline(always)]
            pub fn $fn(&mut self) {unimplemented!()}
        }
    };
    //
    (self: $lhs:ty, output: $output:ty, fn: $fn:ident, perform: $action:ident | 3) => {
        impl $lhs {
            #[inline(always)]
            pub fn $fn(self) -> $output {
                <$output>::new(
                    self[0].$action(),
                    self[1].$action(),
                    self[2].$action()
                )
            }
        }
    };
    (ref self: $lhs:ty, output: $output:ty, fn: $fn:ident, perform: $action:ident | 3) => {
        impl $lhs {
            #[inline(always)]
            pub fn $fn(&self) -> $output {
                <$output>::new(
                    self[0].$action(),
                    self[1].$action(),
                    self[2].$action(),
                )
            }
        }
    };
    (ref mut self: $lhs:ty, output: $output:ty, fn: $fn:ident, perform: $action:ident | 3) => {
        impl $lhs {
            #[inline(always)]
            pub fn $fn(&mut self) -> $output {unimplemented!()}
        }
    };
    //
    // cimpl!(self: Vec3, output: Vec3, fn: clamp, perform: clamp, args: min:f32, max:f32 | 3);
    (self: $lhs:ty, output: $output:ty, fn: $fn:ident, perform: $action:ident, args: $($arg_ident:ident:$arg_ty:ty, )+ | 3) => {
        impl $lhs {
            #[inline(always)]
            pub fn $fn(self, $($arg_ident:$arg_ty, )+) -> $output {
                <$output>::new(
                    self[0].$action($($arg_ident, )+),
                    self[1].$action($($arg_ident, )+),
                    self[2].$action($($arg_ident, )+)
                )
            }
        }
    };
    (ref self: $lhs:ty, output: $output:ty, fn: $fn:ident, perform: $action:ident, args: $($arg_ident:ident:$arg_ty:ty, )+ | 3) => {
        impl $lhs {
            #[inline(always)]
            pub fn $fn(&self, $($arg_ident:$arg_ty)+) -> $output  {unimplemented!()}
        }
    };
    (ref mut self: $lhs:ty, output: $output:ty, fn: $fn:ident, perform: $action:ident, args: $($arg_ident:ident:$arg_ty:ty, )+ | 3) => {
        impl $lhs {
            #[inline(always)]
            pub fn $fn(&mut self, $($arg_ident:$arg_ty)+) -> $output  {unimplemented!()}
        }
    };
}

// regex for searching using template = [$::A-Za-z]*
// dont forget to \ the | to \|
