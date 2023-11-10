use num_traits::{identities::Zero, Euclid, Float, Pow};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

pub(crate) trait NumCast<T>: Sized {
    fn from(n: T) -> Self;
}

macro_rules! permit_cast {
    ($t1:ty as $t2:ty) => {
        impl NumCast<$t1> for $t2 {
            #[inline]
            fn from(n: $t1) -> $t2 {
                n as $t2
            }
        }
    };
}

macro_rules! permit_cast_to_all_primitive_numeric_types_for {
    ($($t1:ident)*)=>{
        $(
            permit_cast!($t1 as i8);
            permit_cast!($t1 as i16);
            permit_cast!($t1 as i32);
            permit_cast!($t1 as i64);
            permit_cast!($t1 as i128);
            permit_cast!($t1 as u8);
            permit_cast!($t1 as u16);
            permit_cast!($t1 as u32);
            permit_cast!($t1 as u64);
            permit_cast!($t1 as u128);
            permit_cast!($t1 as isize);
            permit_cast!($t1 as usize);
            permit_cast!($t1 as f32);
            permit_cast!($t1 as f64);
        )*
    }
}
permit_cast_to_all_primitive_numeric_types_for! {i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize f32 f64}

macro_rules! impl_vector {
    ($name:ident, $dim:literal, $($xi:literal:$x:ident),+) => {
        #[derive(Clone, Copy, PartialEq, Eq)]
        pub(crate) struct $name<T> {
            $(pub(crate) $x: T),+
        }

        impl<T> $name<T> {
            #[inline]
            pub(crate) fn new($($x: T),+) -> Self {
                Self { $($x),+ }
            }

            #[inline]
            pub(crate) fn cast<T2>(self) -> $name<T2> where T2: NumCast<T> {
                $name::new($(T2::from(self.$x),)+)
            }

            #[inline]
            pub(crate) fn map<F>(self, f: F) -> Self where F: Fn(T) -> T {
                Self { $($x: f(self.$x)),+ }
            }

            #[inline]
            pub(crate) fn floor(self) -> Self where T: Float {
                Self { $($x: self.$x.floor()),+ }
            }

            #[inline]
            pub(crate) fn rem_euclid(self, rhs: T) -> Self where T: Euclid {
                Self { $($x: self.$x.rem_euclid(&rhs)),+ }
            }

            #[inline]
            pub(crate) fn sum(self) -> T where T: Pow<i32, Output = T> + Zero + AddAssign {
                let mut result = T::zero();
                $(result += self.$x;)+
                result
            }

            #[inline]
            pub(crate) fn norm_l2_squared(self) -> T where T: Pow<i32, Output = T> + Zero + AddAssign {
                let mut result = T::zero();
                $(result += self.$x.pow(2);)+
                result
            }
        }

        impl_op!(+, Add, add, $name, $($x),+);
        impl_op!(-, Sub, sub, $name, $($x),+);
        impl_op!(*, Mul, mul, $name, $($x),+);
        impl_op!(/, Div, div, $name, $($x),+);

        impl_op_assign!(+=, AddAssign, add_assign, $name, $($x),+);
        impl_op_assign!(-=, SubAssign, sub_assign, $name, $($x),+);
        impl_op_assign!(*=, MulAssign, mul_assign, $name, $($x),+);
        impl_op_assign!(/=, DivAssign, div_assign, $name, $($x),+);

        impl<T: Copy> From<[T; $dim]> for $name<T> {
            #[inline]
            fn from(value: [T; $dim]) -> Self {
                Self { $($x: value[$xi]),+ }
            }
        }
    }
}

macro_rules! impl_op {
    ($op:tt, $optrait:ident, $opfun:ident, $name:ident, $($x:ident),+) => {
        impl<T: $optrait<Output = T>> $optrait for $name<T> {
            type Output = Self;
            #[inline]
            fn $opfun(self, rhs: Self) -> Self::Output {
                Self { $($x: self.$x $op rhs.$x),+ }
            }
        }

        impl<T: $optrait<Output = T> + Copy> $optrait<T> for $name<T> {
            type Output = Self;
            #[inline]
            fn $opfun(self, rhs: T) -> Self::Output {
                Self { $($x: self.$x $op rhs),+ }
            }
        }
    }
}

macro_rules! impl_op_assign {
    ($op:tt, $optrait:ident, $opfun:ident, $name:ident, $($x:ident),+) => {
        impl<T: $optrait> $optrait for $name<T> {
            #[inline]
            fn $opfun(&mut self, rhs: Self) {
                $(self.$x $op rhs.$x;)+
            }
        }

        impl<T: $optrait + Copy> $optrait<T> for $name<T> {
            #[inline]
            fn $opfun(&mut self, rhs: T) {
                $(self.$x $op rhs;)+
            }
        }
    }
}

impl_vector!(Vec2, 2, 0: x, 1: y);
impl_vector!(Vec3, 3, 0: x, 1: y, 2: z);
impl_vector!(Vec4, 4, 0: x, 1: y, 2: z, 3: w);
