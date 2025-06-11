use crate::impl_number_type;
impl_number_type!(f64, Float, AsFloat, as_float);


// use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
// use std::convert::{AsMut, AsRef};
// use std::fmt::{Debug, Display, Formatter};
// use std::hash::{Hash, Hasher};
// use std::ops::{Deref, DerefMut};
//
// use crate::AsNumber;
//
// pub trait AsFloat<T>: AsNumber<T> {}
//
// #[derive(Clone, Copy)]
// pub struct Float {
//     value: f64,
// }
//
// impl Float {
//     pub fn inner(&self) -> f64 {
//         self.value
//     }
// }
// impl AsRef<T> for Float {
//     fn as_ref(&self) -> &f64 {
//         &self.value
//     }
// }
// impl AsMut<T> for Float {
//     fn as_mut(&mut self) -> &mut f64 {
//         &mut self.value
//     }
// }
//
// impl Deref for Float {
//     type Target = f64;
//
//     fn deref(&self) -> &f64 {
//         &self.value
//     }
// }
//
// impl DerefMut for Float {
//     fn deref_mut(&mut self) -> &mut f64 {
//         &mut self.value
//     }
// }
//
// impl Display for Float {
//     fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
//         write!(f, "{}", self.value)
//     }
// }
//
// impl Debug for Float {
//     fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
//         write!(f, "{}", self.value)
//     }
// }
//
// impl PartialEq for Float {
//     fn eq(&self, other: &Float) -> bool {
//         self.inner().eq(other.as_ref())
//     }
// }
// impl Eq for Float {}
// impl PartialOrd for Float {
//     fn partial_cmp(&self, other: &Float) -> Option<Ordering> {
//         self.inner().partial_cmp(other.as_ref())
//     }
// }
//
// impl Ord for Float {
//     fn cmp(&self, other: &Self) -> Ordering {
//         if self.is_null() {
//             return Ordering::Less;
//         }
//         self.inner().cmp(other.as_ref())
//     }
// }
//
// impl Hash for Float {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.inner().hash(state)
//     }
// }
//
