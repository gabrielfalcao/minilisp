use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::fmt::{Debug, Display};

use crate::impl_as_number;
pub trait ListValue: Sized + PartialOrd + Ord + PartialEq + Eq + Clone + Debug + Display {}
impl<T: Sized + PartialOrd + Ord + PartialEq + Eq + Clone + Debug + Display> ListValue for T {}

pub trait AsNumber<T>: Sized + PartialOrd + PartialEq + Clone + Debug + Display {
    fn as_number(&self) -> T;
}
impl_as_number!(u8);
impl_as_number!(u16);
impl_as_number!(u32);
impl_as_number!(u64);
impl_as_number!(usize);

impl_as_number!(i8);
impl_as_number!(i16);
impl_as_number!(i32);
impl_as_number!(i64);
impl_as_number!(isize);

impl_as_number!(f32);
impl_as_number!(f64);

#[macro_export]
macro_rules! impl_as_number {
    ($type:ty) => {
        impl AsNumber<$type> for $type {
            fn as_number(&self) -> $type{
                *self
            }
        }
    };
}
