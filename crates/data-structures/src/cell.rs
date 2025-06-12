#![allow(unused)]
use std::iter::{Extend, IntoIterator, Iterator};
use std::ops::Deref;

use unique_pointer::{RefCounter, UniquePointer};

use crate::{AsSymbol, AsValue, Quotable, Symbol, Value};
pub trait ListIterator<'c, T: AsCell<'c>>: IntoIterator<Item = T> {
    fn list_iter(&self) -> Cell<'c>;
}

pub trait AsCell<'c>: Quotable {
    //: ListValue {
    fn as_cell(&self) -> Cell<'c>;
}

#[derive(Eq, PartialOrd, Ord)]
pub struct Cell<'c> {
    pub(crate) head: UniquePointer<Value<'c>>,
    pub(crate) tail: UniquePointer<Cell<'c>>,
    pub(crate) cur: UniquePointer<Cell<'c>>,
    pub(crate) refs: RefCounter,
    pub(crate) quoted: bool,
}

impl<'c> Cell<'c> {
    pub fn nil() -> Cell<'c> {
        Cell::quoted(Option::<Value<'c>>::None, false)
        // Cell {
        //     head: UniquePointer::<Value<'c>>::null(),
        //     tail: UniquePointer::<Cell<'c>>::null(),
        //     refs: RefCounter::null(),
        //     quoted: false
        // }
    }

    pub fn quoted<T: AsValue<'c>>(item: Option<T>, quoted: bool) -> Cell<'c> {
        let mut cell = Cell {
            head: UniquePointer::<Value<'c>>::null(),
            tail: UniquePointer::<Cell<'c>>::null(),
            cur: UniquePointer::<Cell<'c>>::null(),
            refs: RefCounter::null(),
            quoted,
        };
        if let Some(item) = item {
            cell.write(item.as_value());
        }
        cell
    }

    pub fn quote(&self) -> Cell<'c> {
        let mut cell = self.clone();
        cell.quoted = true;
        cell
    }

    pub fn is_nil(&self) -> bool {
        self.head.is_null() && self.tail.is_null()
    }

    pub fn unwrap_value(&self) -> Value<'c> {
        if self.tail.is_null() {
            match self.head() {
                Some(head) => head.unwrap_list(),
                None => Value::Nil,
            }
        } else if self.quoted {
            Value::QuotedList(self.as_cell())
        } else {
            Value::List(self.as_cell())
        }
    }

    pub fn new<T: AsValue<'c>>(item: T) -> Cell<'c> {
        Cell::quoted(Some(item), false)
        // let mut cell = Cell::nil();
        // cell.write(item.as_value());
        // cell
    }

    pub fn head(&self) -> Option<Value<'c>> {
        self.head.try_read()
    }

    pub fn add(&mut self, new: &Cell<'c>) {
        let mut new = new.clone();
        self.incr_ref();

        if self.head.is_null() {
            // when self.head *IS* null:
            // and `new.head` *IS NOT* null
            if !new.head.is_null() {
                // swap heads
                self.swap_head(&mut new);
            }

            // and new.tail *IS NOT* null
            if !new.tail.is_null() {
                let tail = new.tail.inner_mut();
                tail.head.write_ref(new.head.inner_ref());
                self.swap_refs(&mut new);
            }
            self.tail = UniquePointer::from(new);
        } else {
            // when self.head *IS NOT* null
            if self.tail.is_null() {
                // dbg!(&self, &new);
                // when self.tail *IS* null
                self.tail = UniquePointer::from(new);
            } else {
                // dbg!(self.tail.inner_ref(), &self, &new);
                //  self.tail *IS NOT* null
                self.tail.inner_mut().add(&new);
            }
        }
    }

    pub fn pop(&mut self) -> bool {
        if !self.tail.is_null() {
            self.tail.drop_in_place();
            self.tail = UniquePointer::null();
            true
        } else if !self.head.is_null() {
            self.head.drop_in_place();
            self.head = UniquePointer::null();
            true
        } else {
            false
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() > 0
    }

    /// `O(n)`
    pub fn len(&self) -> usize {
        let mut len = 0;
        if !self.head.is_null() {
            len += 1
        }
        if let Some(tail) = self.tail() {
            len += tail.len();
        }
        len
    }

    pub fn tail(&self) -> Option<&'c Cell<'c>> {
        self.tail.as_ref()
    }

    pub fn values(&self) -> Vec<Value<'c>> {
        let mut values = Vec::<Value>::new();
        if let Some(head) = self.head() {
            // dbg!(&head);
            values.push(head.clone());
        }
        if let Some(tail) = self.tail() {
            // dbg!(&tail);
            values.extend(tail.values());
        }
        values
    }

    pub(crate) fn write(&mut self, value: Value<'c>) {
        self.head.write(value);
        self.incr_ref();
        assert!(self.cur.is_null());
        self.cur = UniquePointer::from_ref(self);
    }

    pub(crate) fn swap_head(&mut self, other: &mut Self) {
        self.head = unsafe {
            let head = other.head.propagate();
            other.head = self.head.propagate();
            head
        };
    }

    pub(crate) fn swap_refs(&mut self, other: &mut Self) {
        self.refs = {
            let refs = other.refs.clone();
            other.refs = self.refs.clone();
            refs
        };
    }

    pub fn to_vec(&self) -> Vec<Value<'c>> {
        Vec::<Value<'c>>::from_iter(self.clone().into_iter())
    }

    fn incr_ref(&mut self) {
        self.refs += 1;
        if !self.tail.is_null() {
            if let Some(tail) = self.tail.as_mut() {
                tail.incr_ref();
            }
        }
    }

    fn decr_ref(&mut self) {
        self.refs -= 1;
        if !self.tail.is_null() {
            if let Some(tail) = self.tail.as_mut() {
                tail.decr_ref();
            }
        }
    }

    fn dealloc(&mut self) {
        if self.refs > 0 {
            self.decr_ref();
        } else {
            self.head.drop_in_place();
            self.tail.drop_in_place();
        }
    }
}
impl<'c> Quotable for Cell<'c> {
    fn is_quoted(&self) -> bool {
        self.quoted
    }
}

impl<'c, T: Quotable + AsCell<'c>, const N: usize> AsCell<'c> for [T; N] {
    fn as_cell(&self) -> Cell<'c> {
        let mut cell = Cell::nil();
        for item in self {
            cell.add(&item.as_cell());
        }
        cell
    }
}

// impl<'c> AsList<'c> for Cell<'c> {
//     fn as_cell(&self) -> List<'c> {
//         if self.is_nil() {
//             List::Empty(self.is_quoted())
//         } else {
//             List::Linked(self.clone(), self.is_quoted())
//         }
//     }
// }

impl<'c, T: AsCell<'c>, const N: usize> ListIterator<'c, T> for [T; N] {
    fn list_iter(&self) -> Cell<'c> {
        let mut cell = Cell::nil();
        for item in self {
            cell.add(&item.as_cell());
        }
        cell
    }
}
impl<'c> AsCell<'c> for Cell<'c> {
    fn as_cell(&self) -> Cell<'c> {
        self.clone()
    }
}
impl<'c> AsCell<'c> for &Cell<'c> {
    fn as_cell(&self) -> Cell<'c> {
        UniquePointer::read_only(*self).read()
    }
}

impl<'c> AsCell<'c> for &'c str {
    fn as_cell(&self) -> Cell<'c> {
        Cell::new(Value::symbol(self.to_string()))
    }
}
impl<'c> AsCell<'c> for String {
    fn as_cell(&self) -> Cell<'c> {
        Cell::new(Value::string(self))
    }
}

impl<'c> From<Value<'c>> for Cell<'c> {
    fn from(value: Value<'c>) -> Cell<'c> {
        Cell::new(value)
    }
}

impl<'c> From<u8> for Cell<'c> {
    fn from(value: u8) -> Cell<'c> {
        Cell::new(Value::Byte(value))
    }
}
impl<'c> From<u32> for Cell<'c> {
    fn from(value: u32) -> Cell<'c> {
        if value <= u8::MAX.into() {
            Cell::new(Value::Byte(value as u8))
        } else {
            Cell::new(Value::UnsignedInteger(value.into()))
        }
    }
}
impl<'c> From<f64> for Cell<'c> {
    fn from(value: f64) -> Cell<'c> {
        Cell::new(Value::float(value))
    }
}
impl<'c> From<u64> for Cell<'c> {
    fn from(value: u64) -> Cell<'c> {
        if value <= u32::MAX.into() {
            Cell::from(value as u32)
        } else {
            Cell::new(Value::UnsignedInteger(value.into()))
        }
    }
}
impl<'c> From<i32> for Cell<'c> {
    fn from(value: i32) -> Cell<'c> {
        if let Ok(value) = TryInto::<u32>::try_into(value) {
            Cell::new(Value::unsigned_integer(value))
        } else {
            Cell::new(Value::integer(value))
        }
    }
}
impl<'c> From<i64> for Cell<'c> {
    fn from(value: i64) -> Cell<'c> {
        Cell::new(Value::from(value))
    }
}

impl<'c> PartialEq<Cell<'c>> for Cell<'c> {
    fn eq(&self, other: &Cell<'c>) -> bool {
        if self.head.is_null() == other.head.is_null() {
            true
        } else if let Some(head) = self.head() {
            if let Some(value) = other.head() {
                return head == value && (self.tail() == other.tail());
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl<'c> Default for Cell<'c> {
    fn default() -> Cell<'c> {
        Cell::nil()
    }
}
impl<'c> Clone for Cell<'c> {
    fn clone(&self) -> Cell<'c> {
        let mut cell = Cell::nil();
        cell.refs = self.refs.clone();
        if let Some(head) = self.head() {
            cell.head.write(head)
        }
        if let Some(tail) = self.tail().map(Clone::clone) {
            cell.tail.write(tail)
        }
        cell
    }
}
impl<'c> Drop for Cell<'c> {
    fn drop(&mut self) {
        self.dealloc();
    }
}

impl std::fmt::Debug for Cell<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            [
                "Cell".to_string(),
                format!(
                    "[{}]",
                    if self.is_nil() {
                        format!("null")
                    } else {
                        [
                            if self.head.is_null() {
                                format!("head: {}", "null")
                            } else {
                                format!(
                                    "head={:#?}",
                                    self.head().unwrap_or_default()
                                )
                            },
                            if self.tail.is_null() {
                                format!("tail: {}", "null")
                            } else {
                                format!(
                                    "tail={:#?}",
                                    self.tail()
                                        .map(Clone::clone)
                                        .unwrap_or_default()
                                )
                            },
                        ]
                        .join(" | ")
                    }
                )
            ]
            .join("")
        )
    }
}
impl std::fmt::Display for Cell<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            [
                if self.head.is_null() {
                    String::new()
                } else {
                    self.head().unwrap_or_default().to_string()
                },
                if self.tail.is_null() {
                    String::new()
                } else {
                    self.tail()
                        .map(Clone::clone)
                        .unwrap_or_default()
                        .to_string()
                },
            ]
            .join(" ")
            .trim()
        )
    }
}

// impl<'c> Iterator for Cell<'c> {
//     type Item = Value<'c>;

//     fn next(&mut self) -> Option<Self::Item> {
//         eprintln!();
//         dbg!(&self.cur);
//         if self.cur.is_not_null() {
//             let value = self.cur.inner_ref().head();
//             self.cur = self.cur.inner_ref().tail.clone();
//             dbg!(&self.cur);
//             value
//         } else {
//             // rewind iterator
//             self.cur = UniquePointer::from_ref(self);
//             None
//         }
//     }
// }

impl<'c> AsValue<'c> for Cell<'c> {
    fn as_value(&self) -> Value<'c> {
        if self.tail.is_null() {
            match self.head() {
                Some(head) => head.unwrap_list(),
                None => Value::Nil,
            }
        } else if self.quoted {
            Value::QuotedList(self.clone())
        } else {
            Value::List(self.clone())
        }
    }
}

impl<'c, T> From<T> for Cell<'c>
where
    T: AsSymbol<'c>,
{
    fn from(sym: T) -> Cell<'c> {
        Cell::from(Value::from(sym.as_symbol()))
    }
}

// // // // // impl<'c> From<&'c str> for Cell<'c> {
// // // // //     fn from(value: &'c str) -> Cell<'c> {
// // // // //         let value = Value::from(value);
// // // // //         Cell::new(value)
// // // // //     }
// // // // // }

// impl<'c, 'i> Extend<&'c &'i Value<'c>> for Cell<'c> {
//     fn extend<T: IntoIterator<Item = &'c &'i Value<'c>>>(&mut self, iter: T) {
//         for value in iter {
//             self.add(value);
//         }
//     }
// }

// impl<'c> IntoIterator for &'c Cell<'c> {
//     type IntoIter = Cell<'c>Iter<'c>;
//     type Item = &'c String;

//     fn into_iter(self) -> Self::IntoIter {
//         self.iter()
//     }
// }

// impl<'c> From<Vec<Value<'c>>> for Cell<'c> {
//     fn from(iter: Vec<Value<'c>>) -> Cell<'c> {
//         let mut buf = Cell<'c>::new();
//         buf.extend(iter);
//         buf
//     }
// }
// impl<'c, const N: usize> From<[&'c str; N]> for Cell<'c> {
//     fn from(iter: [&'c str; N]) -> Cell<'c> {
//         let mut buf = Cell<'c>::new();
//         buf.extend(iter);
//         buf
//     }
// }
// impl<'c, 'i> From<&'c [&'i str]> for Cell<'c> {
//     fn from(iter: &'c [&'i str]) -> Cell<'c> {
//         let mut buf = Cell<'c>::new();
//         buf.extend(iter);
//         buf
//     }
// }
// impl<'c, 'i, const N: usize> From<&'c [&'i str; N]> for Cell<'c> {
//     fn from(iter: &'c [&'i str; N]) -> Cell<'c> {
//         let mut buf = Cell<'c>::new();
//         buf.extend(iter);
//         buf
//     }
// }

// impl<'c> Deref for Cell<'c> {
//     type Target = [&Value<'c>];

//     fn deref(&self) -> &[&Value<'c>] {
//         self.to_vec().as_slice()
//     }
// }

pub struct CellIterator<'c> {
    cell: UniquePointer<Cell<'c>>,
}

impl<'c> CellIterator<'c> {
    pub fn new(cell: Cell<'c>) -> CellIterator<'c> {
        CellIterator {
            cell: UniquePointer::from_ref(&cell),
        }
    }

    pub fn item(&self) -> Option<&Cell<'c>> {
        self.cell.as_ref()
    }

    pub fn tail(&self) -> Option<&Cell<'c>> {
        if let Some(cell) = self.cell.as_ref() {
            cell.tail()
        } else {
            None
        }
    }
}
impl<'c> Iterator for CellIterator<'c> {
    type Item = Value<'c>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cell.is_not_null() {
            let value = self.cell.inner_ref().head();
            let next_tail = self.cell.inner_ref().tail.clone();
            self.cell = next_tail;
            value
        } else {
            None
        }
    }
}

impl<'c> IntoIterator for Cell<'c> {
    type IntoIter = CellIterator<'c>;
    type Item = Value<'c>;

    fn into_iter(self) -> Self::IntoIter {
        CellIterator::new(self)
    }
}
