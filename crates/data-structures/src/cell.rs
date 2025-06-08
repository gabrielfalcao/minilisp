use std::borrow::{Borrow, Cow, ToOwned};
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::convert::AsRef;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::{Extend, FromIterator, IntoIterator};
use std::marker::PhantomData;
use std::mem::{ManuallyDrop, MaybeUninit};
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::ptr::NonNull;

use crate::{car, cdr, color, cons, internal, step, Value, RefCounter};

/// Rust implementation of lisp's cons cell.
pub struct Cell<'c> {
    head: *mut Value<'c>,
    tail: *mut Cell<'c>,
    refs: RefCounter,
}

impl<'c> PartialOrd for Cell<'c> {
    fn partial_cmp(&self, other: &Cell<'c>) -> Option<Ordering> {
        self.values().partial_cmp(&other.values())
    }
}
impl<'c> PartialEq for Cell<'c> {
    fn eq(&self, other: &Cell<'c>) -> bool {
        self.values().eq(&other.values())
    }
}
impl<'c> Cell<'c> {
    pub fn nil() -> Cell<'c> {
        Cell {
            head: internal::null::value(),
            tail: internal::null::cell(),
            refs: RefCounter::new(),
        }
    }

    pub fn is_nil(&self) -> bool {
        self.head.is_null() && self.tail.is_null()
    }

    pub fn new(value: Value<'c>) -> Cell<'c> {
        let mut cell = Cell::nil();
        unsafe {
            let head = internal::alloc::value();
            head.write(value);
            cell.head = head;
        }
        cell
    }

    pub fn head(&self) -> Option<Value<'c>> {
        let value = if self.head.is_null() {
            None
        } else {
            let head = internal::alloc::value();
            let value = unsafe {
                self.head.copy_to(head);
                head.read()
            };
            Some(value)
        };
        value
    }

    pub fn add(&mut self, mut new: &mut Cell<'c>) {
        if self.head.is_null() {
            unsafe {
                if !new.head.is_null() {
                    self.head = internal::alloc::value();
                    std::ptr::swap(self.head as *mut Value<'c>, new.head as *mut Value<'c>);
                }

                if !new.tail.is_null() {
                    let refs = new.refs;
                    let mut tail = new.tail.read();
                    let head = internal::alloc::value();
                    if !tail.head.is_null() {
                        head.write(tail.head.read());
                    }
                    new.head = head;
                    self.refs = refs;
                }
            }
        } else {
            new.incr_ref();
            self.incr_ref();
            if self.tail.is_null() {
                unsafe {
                    let mut new_tail = std::ptr::from_mut::<Cell<'c>>(new);
                    self.tail = new_tail;
                }
            } else {
                unsafe {
                    let mut tail = &mut *self.tail;
                    tail.add(new);
                }
            }
        }
    }

    pub fn pop(&mut self) -> bool {
        if !self.tail.is_null() {
            unsafe {
                self.tail = internal::null::cell();
            }
            true
        } else if !self.head.is_null() {
            self.head = internal::null::value();
            true
        } else {
            false
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() > 0
    }

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
        if self.tail.is_null() {
            None
        } else {
            unsafe {
                if let Some(tail) = self.tail.as_ref() {
                    Some(tail)
                } else {
                    None
                }
            }
        }
    }

    // pub fn list(&self) -> List {
    //     if self.is_nil() {
    //         List::Empty
    //     } else if self.head.is_null() {
    //         self.tail().unwrap().list()
    //     } else {
    //         let car = self.head().unwrap();
    //         if self.tail.is_null() {
    //             List::Car(car)
    //         } else {
    //             List::Cons(car, self.tail().unwrap())
    //         }
    //     }
    // }

    pub fn values(&self) -> Vec<Value<'c>> {
        let mut values = Vec::<Value>::new();
        if let Some(head) = self.head() {
            values.push(head.clone());
        }
        if let Some(tail) = self.tail() {
            values.extend(tail.values());
        }
        values
    }

    fn incr_ref(&mut self) {
        self.refs += 1;
        if !self.tail.is_null() {
            unsafe {
                let mut tail = self.tail as *mut Cell<'c>;
                if let Some(mut tail) = tail.as_mut() {
                    tail.refs += 1;
                }
            }
        }
    }
}

impl<'c> From<Value<'c>> for Cell<'c> {
    fn from(value: Value<'c>) -> Cell<'c> {
        Cell::new(value)
    }
}
impl<'c> From<&'c str> for Cell<'c> {
    fn from(value: &'c str) -> Cell<'c> {
        let value = Value::from(value);
        Cell::new(value)
    }
}
impl<'c> From<u8> for Cell<'c> {
    fn from(value: u8) -> Cell<'c> {
        Cell::new(Value::Byte(value))
    }
}
impl<'c> From<u64> for Cell<'c> {
    fn from(value: u64) -> Cell<'c> {
        if value < u8::MAX.into() {
            Cell::new(Value::Byte(value as u8))
        } else {
            Cell::new(Value::UInt(value))
        }
    }
}
impl<'c> From<i32> for Cell<'c> {
    fn from(value: i32) -> Cell<'c> {
        if let Ok(value) = TryInto::<u64>::try_into(value) {
            Cell::new(Value::UInt(value))
        } else {
            Cell::new(Value::Int(value.into()))
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
        unsafe {
            if !self.head.is_null() {
                let head = internal::alloc::value();
                head.write(self.head.read());
                cell.head = head;
            }
            if !self.tail.is_null() {
                let tail = internal::alloc::cell();
                tail.write(self.tail.read());
                cell.tail = tail;
            }
        }
        cell
    }
}
impl<'c> Drop for Cell<'c> {
    fn drop(&mut self) {
        if self.refs > 0 {
            self.refs -= 1;
        } else {
            unsafe {
                internal::dealloc::value(self.head);
                internal::dealloc::cell(self.tail);
            }
        }
    }
}

impl std::fmt::Debug for Cell<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}[head:{} | tail:{}]",
            crate::color::reset(""),
            crate::color::fg("Cell", 87),
            crate::color::fg("@", 231),
            crate::color::ptr_inv(self),
            if self.head.is_null() {
                color::fore("null", 196)
            } else {
                color::ptr(self.head)
            },
            if self.tail.is_null() {
                color::fore("null", 196)
            } else {
                color::ptr(self.tail)
            },
        )
    }
}

impl std::fmt::Display for Cell<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", {
            let values = self.values();
            match values.len() {
                0 => "nil".to_string(),
                1 => values[0].to_string(),
                _ => format!(
                    "({})",
                    values.iter().map(|value| value.to_string()).collect::<Vec<String>>().join(" ")
                ),
            }
        })
    }
}
