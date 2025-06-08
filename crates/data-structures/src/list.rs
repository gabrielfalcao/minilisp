use std::borrow::{Borrow, Cow, ToOwned};
use std::convert::AsRef;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::{Extend, FromIterator, IntoIterator};
use std::marker::PhantomData;
use std::mem::{ManuallyDrop, MaybeUninit};
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::ptr::NonNull;

use crate::{color, Cell, Value};

/// Rust abstraction of lisp lists
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub enum List<'c> {
    Empty,
    Car(Value<'c>),
    Cons(Value<'c>, Cell<'c>),
}

impl<'c> List<'c> {
    pub fn is_nil(&self) -> bool {
        *self == List::Empty
    }

    pub fn is_empty(&self) -> bool {
        self.is_nil()
    }
}

impl<'c> From<Value<'c>> for List<'c> {
    fn from(value: Value<'c>) -> List<'c> {
        List::new(value)
    }
}
impl<'c> From<&'c str> for List<'c> {
    fn from(value: &'c str) -> List<'c> {
        let value = Value::from(value);
        List::new(value)
    }
}
impl<'c> From<u8> for List<'c> {
    fn from(value: u8) -> List<'c> {
        List::new(Value::Byte(value))
    }
}
impl<'c> From<u64> for List<'c> {
    fn from(value: u64) -> List<'c> {
        if value < u8::MAX.into() {
            List::new(Value::Byte(value as u8))
        } else {
            List::new(Value::UInt(value))
        }
    }
}
impl<'c> From<i32> for List<'c> {
    fn from(value: i32) -> List<'c> {
        if let Ok(value) = TryInto::<u64>::try_into(value) {
            List::new(Value::UInt(value))
        } else {
            List::new(Value::Int(value.into()))
        }
    }
}
impl<'c> From<i64> for List<'c> {
    fn from(value: i64) -> List<'c> {
        List::new(Value::from(value))
    }
}

impl<'c> PartialEq<List<'c>> for List<'c> {
    fn eq(&self, other: &List<'c>) -> bool {
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

impl<'c> Default for List<'c> {
    fn default() -> List<'c> {
        List::nil()
    }
}
impl<'c> Clone for List<'c> {
    fn clone(&self) -> List<'c> {
        let mut cell = List::nil();
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
impl<'c> Drop for List<'c> {
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

impl std::fmt::Display for List<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                List::Empty => "nil".to_string(),
                List::Car(car) => car.to_string(),
                List::Cons(car, cell) => {
                    if cell.tail.is_null() {
                        format!("({})", car)
                    } else {
                        format!("({} {})", car, cell.values().join(" "))
                    }
                },
            }
        )
    }
}
