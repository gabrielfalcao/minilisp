#![allow(unused)]
use std::borrow::Cow;
use std::convert::{AsMut, AsRef};
use std::fmt::{Debug, Display, Formatter};

use unique_pointer::UniquePointer;

pub mod integer;
mod macros;
pub use integer::{AsInteger, Integer};
pub mod float;
pub use float::{AsFloat, Float};
pub mod unsigned_integer;
pub use unsigned_integer::{AsUnsignedInteger, UnsignedInteger};

use crate::{AsCell, AsNumber, AsSymbol, Cell, Quotable, Symbol};

pub trait AsValue<'c>: Quotable {
    fn as_value(&self) -> Value<'c>;
}

#[derive(Clone, PartialOrd, Ord, Default, PartialEq, Eq)]
pub enum Value<'c> {
    #[default]
    Nil,
    T,
    String(Cow<'c, str>),
    Symbol(Symbol<'c>),
    QuotedSymbol(Symbol<'c>),
    Byte(u8),
    UnsignedInteger(UnsignedInteger),
    Integer(Integer),
    Float(Float),
    List(Cell<'c>),
    QuotedList(Cell<'c>),
    EmptyList,
    EmptyQuotedList,
}
impl<'c> Value<'c> {
    pub fn nil() -> Value<'c> {
        Value::Nil
    }

    pub fn t() -> Value<'c> {
        Value::T
    }

    pub fn symbol<T: AsSymbol<'c>>(sym: T) -> Value<'c> {
        Value::Symbol(sym.as_symbol())
    }

    pub fn string<T: ToString>(value: T) -> Value<'c> {
        Value::String(Cow::from(value.to_string()))
    }

    pub fn byte<T: AsNumber<u8>>(byte: T) -> Value<'c> {
        Value::Byte(byte.as_number())
    }

    pub fn unsigned_integer<T: AsUnsignedInteger>(value: T) -> Value<'c> {
        Value::UnsignedInteger(value.as_unsigned_integer())
    }

    pub fn integer<T: AsInteger>(value: T) -> Value<'c> {
        Value::Integer(value.as_integer())
    }

    pub fn float<T: AsFloat>(value: T) -> Value<'c> {
        Value::Float(value.as_float())
    }

    pub fn list<T: AsCell<'c>>(item: T) -> Value<'c> {
        if item.is_quoted() {
            Value::QuotedList(item.as_cell())
        } else {
            Value::List(item.as_cell())
        }
    }

    pub fn is_nil(&self) -> bool {
        if *self == Value::Nil {
            true
        } else {
            false
        }
    }

    // pub(crate) fn extended<'l>(&self) -> Value<'l> {
    //     let value = match self {
    //         Value::String(value) => {
    //             // String
    //             Value::String(value.clone())
    //         },
    //         Value::Symbol(value) => {
    //             // Symbol
    //             Value::Symbol(value.clone())
    //         },
    //         Value::QuotedSymbol(value) => {
    //             // QuotedSymbol
    //             Value::QuotedSymbol(value.clone())
    //         },
    //         Value::Byte(value) => {
    //             // Byte
    //             Value::Byte(value.clone())
    //         },
    //         Value::UnsignedInteger(value) => {
    //             // UnsignedInteger
    //             Value::UnsignedInteger(value.clone())
    //         },
    //         Value::Integer(value) => {
    //             // Integer
    //             Value::Integer(value.clone())
    //         },
    //         Value::Float(value) => {
    //             // Float
    //             Value::Float(value.clone())
    //         },
    //         Value::List(value) => {
    //             // List
    //             Value::List(value.clone())
    //         },
    //         Value::QuotedList(value) => {
    //             // QuotedList
    //             Value::QuotedList(value.clone())
    //         },
    //         Value::Nil => {
    //             // Nil
    //             Value::Nil
    //         },
    //         Value::T => {
    //             // T
    //             Value::T
    //         },
    //         Value::EmptyList => {
    //             // EmptyList
    //             Value::EmptyList
    //         },
    //         Value::EmptyQuotedList => {
    //             // EmptyQuotedList
    //             Value::EmptyQuotedList
    //         },
    //     };
    //     unsafe { std::mem::transmute::<Value<'_>, Value<'l>>(value) }
    // }

    pub fn quote(&self) -> Value<'c> {
        let value = match self {
            Value::Symbol(h) => {
                assert!(!h.is_quoted());
                Value::QuotedSymbol(h.quote())
            },
            Value::List(h) => {
                assert!(!h.is_quoted());
                Value::QuotedList(h.quote())
            },
            Value::QuotedSymbol(h) => {
                assert!(h.is_quoted());
                Value::QuotedSymbol(h.quote())
            },
            Value::QuotedList(h) => {
                assert!(h.is_quoted());
                Value::QuotedList(h.quote())
            },
            // _ => self.clone(),
            _ => self.clone(),
        };
        value.clone()
    }

    pub fn values(&self) -> Vec<Value<'c>> {
        match self {
            Value::List(cell) | Value::QuotedList(cell) => cell.values(),
            _ => Vec::new(),
        }
    }

    pub fn head(&self) -> Value<'c> {
        match self {
            Value::List(cell) | Value::QuotedList(cell) =>
                cell.head().unwrap_or_default(),
            _ => Value::nil(),
        }
    }

    pub fn tail(&self) -> Cell<'c> {
        match self {
            Value::List(cell) | Value::QuotedList(cell) =>
                cell.tail().map(Clone::clone).unwrap_or_default(),
            _ => Cell::nil(),
        }
    }

    pub fn wrap_in_list(&self) -> Value<'c> {
        let value = if self.is_quoted() {
            Value::QuotedList(self.as_cell())
        } else {
            Value::List(self.as_cell())
        };
        value.clone()
    }

    pub fn unwrap_list(&self) -> Value<'c> {
        match self {
            Value::List(cell) | Value::QuotedList(cell) => {
                if cell.tail.is_null() {
                    let value = cell.head().unwrap_or_default();
                    value.clone()
                } else {
                    self.clone()
                }
            },
            _ => self.clone(),
        }
    }
}

impl<'c> Quotable for Value<'c> {
    fn is_quoted(&self) -> bool {
        match self {
            Value::Symbol(h) => {
                assert!(!h.is_quoted());
                false
            },
            Value::List(h) => {
                assert!(!h.is_quoted());
                false
            },
            Value::QuotedSymbol(h) => {
                assert!(h.is_quoted());
                true
            },
            Value::QuotedList(h) => {
                assert!(h.is_quoted());
                true
            },
            Value::EmptyQuotedList => true,
            _ => false,
        }
    }
}

impl<'c> AsValue<'c> for Value<'c> {
    fn as_value(&self) -> Value<'c> {
        self.clone()
    }
}
impl<'c> AsValue<'c> for &Value<'c> {
    fn as_value(&self) -> Value<'c> {
        UniquePointer::read_only(*self).read()
    }
}

impl<'c> Drop for Value<'c> {
    fn drop(&mut self) {}
}

impl Display for Value<'_> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::T => "t".to_string(),
                Value::Nil => "nil".to_string(),
                Value::Byte(h) => format!("{}", h),
                Value::Float(h) => format!("{}", h),
                Value::Integer(h) => format!("{}", h),
                Value::String(h) => format!("{}", h),
                Value::Symbol(h) => format!("{}", h),
                Value::UnsignedInteger(h) => format!("{}", h),
                Value::List(h) => format!("{}", h),
                Value::QuotedList(h) => format!("'{}", h),
                Value::QuotedSymbol(h) => format!("'{}", h),
                Value::EmptyList => format!("()"),
                Value::EmptyQuotedList => format!("'()"),
            }
        )
    }
}
impl Debug for Value<'_> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::Nil => "nil".to_string(),
                Value::T => "t".to_string(),
                Value::String(h) => format!("{:#?}", h),
                Value::Byte(h) => format!("{}u8", h),
                Value::UnsignedInteger(h) => format!("{}", h),
                Value::Integer(h) => format!("{}", h),
                Value::Float(h) => format!("{}", h),
                Value::Symbol(h) => format!("{}", h),
                Value::List(h) => format!("{}", h),
                Value::QuotedList(h) => format!("'{}", h),
                Value::QuotedSymbol(h) => format!("'{}", h),
                Value::EmptyList => format!("()"),
                Value::EmptyQuotedList => format!("'()"),
            }
        )
    }
}
// impl<'c> Clone for Value<'c> {
//     fn clone(&self) -> Value<'c> {
//         self.clone()
//     }
// }
impl<'c> From<()> for Value<'c> {
    fn from(_: ()) -> Value<'c> {
        Value::Nil
    }
}
impl<'c> From<bool> for Value<'c> {
    fn from(value: bool) -> Value<'c> {
        if value {
            Value::T
        } else {
            Value::Nil
        }
    }
}
impl<'c> From<u8> for Value<'c> {
    fn from(value: u8) -> Value<'c> {
        Value::Byte(value)
    }
}
impl<'c> From<Symbol<'c>> for Value<'c> {
    fn from(value: Symbol<'c>) -> Value<'c> {
        Value::Symbol(value)
    }
}
impl<'c> From<&'c str> for Value<'c> {
    fn from(value: &'c str) -> Value<'c> {
        Value::String(Cow::from(value))
    }
}
impl<'c> From<u64> for Value<'c> {
    fn from(value: u64) -> Value<'c> {
        if value < u8::MAX.into() {
            Value::Byte(value as u8)
        } else {
            Value::UnsignedInteger(value.into())
        }
    }
}
impl<'c> From<i32> for Value<'c> {
    fn from(value: i32) -> Value<'c> {
        if let Ok(value) = TryInto::<u64>::try_into(value) {
            Value::UnsignedInteger(value.into())
        } else {
            Value::Integer(value.into())
        }
    }
}

impl<'c> From<Cow<'c, str>> for Value<'c> {
    fn from(value: Cow<'c, str>) -> Value<'c> {
        Value::from(value.into_owned())
    }
}
impl<'c> From<&'c mut str> for Value<'c> {
    fn from(value: &'c mut str) -> Value<'c> {
        Value::String(Cow::<'c, str>::Borrowed(&*value))
    }
}
impl<'c> From<String> for Value<'c> {
    fn from(value: String) -> Value<'c> {
        Value::String(Cow::from(value))
    }
}
impl<'c> From<Option<String>> for Value<'c> {
    fn from(value: Option<String>) -> Value<'c> {
        match value {
            None => Value::Nil,
            Some(value) => Value::from(value),
        }
    }
}
impl<'c> From<Cell<'c>> for Value<'c> {
    fn from(cell: Cell<'c>) -> Value<'c> {
        cell.as_value()
    }
}
// impl<'c> From<List<'c>> for Value<'c> {
//     fn from(list: List<'c>) -> Value<'c> {
//         list.as_value()
//     }
// }

impl<'c> AsRef<Value<'c>> for Value<'c> {
    fn as_ref(&self) -> &Value<'c> {
        &*self
    }
}
impl<'c> AsMut<Value<'c>> for Value<'c> {
    fn as_mut(&mut self) -> &mut Value<'c> {
        &mut *self
    }
}

impl<'c> PartialEq<&Value<'c>> for Value<'c> {
    fn eq(&self, other: &&Value<'c>) -> bool {
        let other = &**other;
        self == other
    }
}
impl<'c> PartialEq<Option<Value<'c>>> for Value<'c> {
    fn eq(&self, other: &Option<Value<'c>>) -> bool {
        match other {
            Some(value) => value.eq(self),
            None => Value::Nil == self,
        }
    }
}
impl<'c> PartialEq<Cell<'c>> for Value<'c> {
    fn eq(&self, other: &Cell<'c>) -> bool {
        other.as_value() == self
    }
}
// impl<'c> PartialEq<List<'c>> for Value<'c> {
//     fn eq(&self, other: &List<'c>) -> bool {
//         other.as_value() == self
//     }
// }

impl<'c> PartialEq<&mut Value<'c>> for Value<'c> {
    fn eq(&self, other: &&mut Value<'c>) -> bool {
        let other = &**other;
        self == other
    }
}

impl<'c> AsValue<'c> for () {
    fn as_value(&self) -> Value<'c> {
        Value::Nil
    }
}
impl<'c> AsValue<'c> for bool {
    fn as_value(&self) -> Value<'c> {
        if *self {
            Value::T
        } else {
            Value::Nil
        }
    }
}
impl<'c> AsValue<'c> for u8 {
    fn as_value(&self) -> Value<'c> {
        Value::Byte(*self)
    }
}
// impl<'c> From<&'static str> for Value<'c> {
//     fn from(value: &'static str) -> Value<'c> {
//         Value::Symbol(value)
//     }
// }
impl<'c> AsValue<'c> for &'c str {
    fn as_value(&self) -> Value<'c> {
        Value::String(Cow::from(self.to_string()))
    }
}
impl<'c> AsValue<'c> for u64 {
    fn as_value(&self) -> Value<'c> {
        if *self < u8::MAX.into() {
            Value::Byte(*self as u8)
        } else {
            Value::UnsignedInteger(self.as_unsigned_integer())
        }
    }
}
impl<'c> AsValue<'c> for i32 {
    fn as_value(&self) -> Value<'c> {
        if let Ok(value) = TryInto::<u64>::try_into(*self) {
            Value::UnsignedInteger(value.as_unsigned_integer())
        } else {
            Value::Integer(self.as_integer())
        }
    }
}

impl<'c> AsValue<'c> for Cow<'c, str> {
    fn as_value(&self) -> Value<'c> {
        Value::from(self.clone().into_owned())
    }
}
impl<'c> AsValue<'c> for &'c mut str {
    fn as_value(&self) -> Value<'c> {
        Value::String(Cow::from(self.to_string()))
    }
}
impl<'c> AsValue<'c> for String {
    fn as_value(&self) -> Value<'c> {
        Value::String(Cow::from(self.to_string()))
    }
}
impl<'c> AsValue<'c> for Option<String> {
    fn as_value(&self) -> Value<'c> {
        match self.clone() {
            None => Value::Nil,
            Some(value) => Value::from(value),
        }
    }
}

impl<'c> AsCell<'c> for Value<'c> {
    fn as_cell(&self) -> Cell<'c> {
        match self {
            Value::Symbol(h) => Cell::quoted(Some(h.unquote()), false),
            Value::QuotedSymbol(h) => Cell::quoted(Some(h.quote()), true),
            Value::List(h) => h.clone(),
            Value::QuotedList(h) => h.clone(),
            _ => Cell::new(self.clone()),
        }
    }
}
impl<'c> AsCell<'c> for &Value<'c> {
    fn as_cell(&self) -> Cell<'c> {
        let value = UniquePointer::read_only(*self).read();
        match &value {
            Value::Symbol(h) => Cell::quoted(Some(h.unquote()), false),
            Value::QuotedSymbol(h) => Cell::quoted(Some(h.quote()), true),
            Value::List(h) => h.as_cell(),
            Value::QuotedList(h) => h.as_cell(),
            _ => Cell::new(value),
        }
    }
}
// impl<'c> AsCell<'c> for Value<'c> {
//     fn as_cell(&self) -> Cell<'c> {
//         match self {
//             Value::Nil => Cell::nil(),
//             Value::T => Cell::from(Value::T),
//             Value::String(value) => Cell::from(Value::string(value)),
//             Value::Symbol(value) => Cell::from(Value::symbol(value)),
//             Value::Byte(value) => Cell::from(Value::byte(*value)),
//             Value::UnsignedInteger(value) =>
//                 Cell::from(Value::unsigned_integer(*value)),
//             Value::Integer(value) => Cell::from(Value::integer(*value)),
//             Value::Float(value) => Cell::from(Value::float(*value)),
//             Value::List(value) => Cell::from(Value::List(value.clone())),
//             Value::QuotedList(value) =>
//                 Cell::from(Value::QuotedList(value.clone())).quote(),
//             Value::QuotedSymbol(value) =>
//                 Cell::from(Value::symbol(value).quote()).quote(),
//             Value::EmptyList => Cell::nil(),
//             Value::EmptyQuotedList => Cell::nil().quote(),
//         }
//     }
// }
