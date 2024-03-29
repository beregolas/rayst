
#[macro_export]
macro_rules! vec_op {
    // setup for easier calls
    ($type:tt, +, $($field:tt)*) => {
        vec_op!(f32, $type, Add, add, AddAssign, add_assign, +, $($field)*);
    };
    ($type:tt, -, $($field:tt)*) => {
        vec_op!(f32, $type, Sub, sub, SubAssign, sub_assign, -, $($field)*);
    };
    ($type:tt, *, $($field:tt)*) => {
        vec_op!(f32, $type, Mul, mul, MulAssign, mul_assign, *, $($field)*);
    };
    ($type:tt, /, $($field:tt)*) => {
        vec_op!(f32, $type, Div, div, DivAssign, div_assign, /, $($field)*);
    };
    // implementation
    ($base_type: tt, $type:tt,
    $op_name:ident, $fn_name:ident,
    $ass_op_name:ident, $ass_fn_name:ident,
    $op:tt,
    $($field:tt)*) => {
        use std::ops::{$op_name, $ass_op_name};

        impl $op_name<$type> for $type {
            type Output = $type;

            fn $fn_name(self, rhs: $type) -> Self::Output {
                $type {
                    $($field: self.$field $op rhs.$field,)*
                }
            }
        }

        impl $op_name<$base_type> for $type {
            type Output = $type;

            fn $fn_name(self, rhs: $base_type) -> Self::Output {
                $type {
                    $($field: self.$field $op rhs,)*
                }
            }
        }

        impl $op_name<$type> for $base_type {
            type Output = $type;

            fn $fn_name(self, rhs: $type) -> Self::Output {
                $type {
                    $($field: self $op rhs.$field,)*
                }
            }
        }

        impl $ass_op_name<$type> for $type {
            fn $ass_fn_name(&mut self, rhs: $type) {
                *self = $type {
                    $($field: self.$field $op rhs.$field,)*
                }
            }
        }

        impl $ass_op_name<$base_type> for $type {
            fn $ass_fn_name(&mut self, rhs: $base_type) {
                *self = $type {
                    $($field: self.$field $op rhs,)*
                }
            }
        }

    }
}

