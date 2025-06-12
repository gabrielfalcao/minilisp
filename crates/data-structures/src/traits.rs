#![allow(unused)]
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::fmt::{Debug, Display};

use crate::{
    impl_as_number, impl_quotable_false, impl_quotable_false_t,
    impl_quotable_false_tn, impl_quotable_string,
};

#[rustfmt::skip]
pub trait ListValue:Sized + PartialOrd + Ord + PartialEq + Eq + Clone + Debug + Display{}
#[rustfmt::skip]
impl<T: Sized + PartialOrd + Ord + PartialEq + Eq + Clone + Debug + Display>ListValue for T{}

pub trait Quotable {
    fn is_quoted(&self) -> bool;
}

impl<T> Quotable for &T
where
    T: Quotable,
{
    fn is_quoted(&self) -> bool {
        Quotable::is_quoted(*self)
    }
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

impl_quotable_false!((), bool, u8, u32, u64, i32, i64);

impl_quotable_false_tn!([T; N]);
impl_quotable_string!(&str, &mut str, String, std::borrow::Cow<'_, str>);

impl Quotable for Option<String> {
    fn is_quoted(&self) -> bool {
        if let Some(string) = self {
            string.starts_with('\'')
        } else {
            false
        }
    }
}

// impl Quotable for () {
//     fn is_quoted(&self) -> bool {
//         false
//     }
// }
// impl Quotable for bool {
//     fn is_quoted(&self) -> bool {
//         false
//     }
// }
// impl Quotable for bool {
//     fn is_quoted(&self) -> bool {
//         false
//     }
// }

pub trait AsNumber<T>:
    Sized + PartialOrd + PartialEq + Clone + Debug + Display
{
    fn as_number(&self) -> T;
}

#[macro_export]
macro_rules! impl_as_number {
    ($type:ty) => {
        impl AsNumber<$type> for $type {
            fn as_number(&self) -> $type {
                *self
            }
        }
        // impl <T>AsNumber<T> for T where T: AsRef<T> {
        //     fn as_number(&self) -> $type{
        //         **self
        //     }
        // }
    };
}

#[macro_export]
macro_rules! impl_quotable_false {
    ($($type:ty),* $(,)?) => {
        $(
            impl Quotable for $type {
                fn is_quoted(&self) -> bool {
                    false
                }
            }
        )*
    };
}
#[macro_export]
macro_rules! impl_quotable_false_t {
    ($type:tt) => {
        impl<T> Quotable for $type {
            fn is_quoted(&self) -> bool {
                false
            }
        }
    };
}
#[macro_export]
macro_rules! impl_quotable_false_tn {
    ($type:tt) => {
        impl<T: Quotable, const N: usize> Quotable for $type {
            fn is_quoted(&self) -> bool {
                for item in self {
                    if item.is_quoted() {
                        return true;
                    }
                }
                false
            }
        }
    };
}

#[macro_export]
macro_rules! impl_quotable_string {
    ($($type:ty),* $(,)?) => {
        $(
            impl Quotable for $type {
                fn is_quoted(&self) -> bool {
                    self.starts_with('\'')
                }
            }
        )*
    };
}
