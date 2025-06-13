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

pub trait ValueListIterator<'c>: IntoIterator<Item = Value<'c>> {
    fn list_value_iter(&self) -> Value<'c>;
}
pub trait ValueQuotedListIterator<'c>: IntoIterator<Item = Value<'c>> {
    fn quoted_list_value_iter(&self) -> Value<'c>;
}

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

    pub fn quoted_symbol<T: AsSymbol<'c>>(sym: T) -> Value<'c> {
        Value::QuotedSymbol(sym.as_symbol())
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

    pub fn quoted_list<T: AsCell<'c>>(item: T) -> Value<'c> {
        Value::QuotedList(item.as_cell())
    }

    pub fn is_nil(&self) -> bool {
        if *self == Value::Nil {
            true
        } else {
            false
        }
    }

    pub fn empty_list() -> Value<'c> {
        Value::EmptyList
    }

    pub fn empty_quoted_list() -> Value<'c> {
        Value::EmptyQuotedList
    }

    pub fn quote(&self) -> Value<'c> {
        let value = match self {
            Value::Symbol(h) => {
                assert!(!h.is_quoted());
                Value::QuotedSymbol(h.clone().quote())
            },
            Value::List(h) => {
                assert!(!h.is_quoted());
                Value::QuotedList(h.clone().quote())
            },
            Value::QuotedSymbol(h) => {
                assert!(h.is_quoted());
                Value::QuotedSymbol(h.clone().quote())
            },
            Value::QuotedList(h) => {
                assert!(h.is_quoted());
                Value::QuotedList(h.clone().quote())
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
        if self.is_quoted() {
            Value::QuotedList(Cell::new(self.clone()))
        } else {
            Value::List(Cell::new(self.clone()))
        }
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
                Value::Byte(h) => format!("0x{:02x}", h),
                Value::Float(h) => format!("{}", h),
                Value::Integer(h) => format!("{}", h),
                Value::String(h) => format!("{:#?}", h),
                Value::Symbol(h) => format!("{}", format!("{}", h)),
                Value::QuotedSymbol(h) => format!("'{}", format!("{}", h)),
                Value::UnsignedInteger(h) => format!("{}", h),
                Value::List(h) => format!("({})", format!("{}", h)),

                Value::QuotedList(h) => format!("'({})", format!("{}", h)),

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
                Value::T => "t".to_string(),
                Value::Nil => "nil".to_string(),
                Value::Byte(h) => format!("0x{:02x}", h),
                Value::Float(h) => format!("{:#?}", h),
                Value::Integer(h) => format!("{:#?}", h),
                Value::String(h) => format!("{:#?}", h),
                Value::Symbol(h) => format!("{}", h),
                Value::QuotedSymbol(h) => format!("'{}", format!("{:#?}", h)),
                Value::UnsignedInteger(h) => format!("{:#?}", h),
                Value::List(h) => format!("({})", format!("{:#?}", h)),
                Value::QuotedList(h) => format!("'({})", format!("{:#?}", h)),
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
            Value::List(h) => {
                let mut cell = Cell::nil();
                for item in h.clone().into_iter() {
                    cell.add(&Cell::new(item));
                }
                dbg!(&cell);
                cell
            },
            Value::QuotedList(h) => {
                let mut cell = Cell::nil().quote();
                for item in h.clone().into_iter() {
                    cell.add(&Cell::new(item));
                }
                dbg!(&cell);
                cell
            },
            _ => Cell::new(self.clone()),
        }
    }
}
impl<'c> AsCell<'c> for &Value<'c> {
    fn as_cell(&self) -> Cell<'c> {
        let value = UniquePointer::read_only(*self).read();
        value.as_cell()
    }
}

// impl<'c, const N: usize> ValueListIterator<'c> for [Value<'c>; N] {
//     fn list_value_iter(&self) -> Value<'c> {
//         let mut cell = Cell::nil();
//         for item in self {
//             cell.add(&Cell::from(item));
//         }
//         Value::list(cell)
//     }
// }
// impl<'c, const N: usize> ValueQuotedListIterator<'c> for [Value<'c>; N] {
//     fn quoted_list_value_iter(&self) -> Value<'c> {
//         let mut cell = Cell::nil();
//         for item in self {
//             cell.add(&Cell::from(item));
//         }
//         Value::quoted_list(cell)
//     }
// }

pub struct ValueIterator<'c> {
    cell: UniquePointer<Cell<'c>>,
}

impl<'c> ValueIterator<'c> {
    pub fn new(cell: &Cell<'c>) -> ValueIterator<'c> {
        ValueIterator {
            cell: UniquePointer::from_ref(cell),
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
impl<'c> Iterator for ValueIterator<'c> {
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

impl<'c> IntoIterator for Value<'c> {
    type IntoIter = ValueIterator<'c>;
    type Item = Value<'c>;

    fn into_iter(self) -> Self::IntoIter {
        ValueIterator::new(&match self {
            Value::List(ref cell) | Value::QuotedList(ref cell) => cell.clone(),
            value => Cell::from(value),
        })
    }
}
impl<'c> Quotable for Value<'c> {
    fn set_quoted(&mut self, quoted: bool) {
        match self {
            Value::Symbol(h) =>
                if quoted {
                    *self = Value::QuotedSymbol(h.clone())
                },
            Value::List(h) =>
                if quoted {
                    *self = Value::QuotedList(h.clone())
                },
            Value::QuotedSymbol(h) =>
                if !quoted {
                    *self = Value::Symbol(h.clone())
                },
            Value::QuotedList(h) =>
                if !quoted {
                    *self = Value::List(h.clone())
                },
            Value::EmptyList =>
                if quoted {
                    *self = Value::EmptyList
                },
            Value::EmptyQuotedList =>
                if quoted {
                    *self = Value::EmptyQuotedList
                },
            _ => {},
        }
    }

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
