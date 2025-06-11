use std::borrow::Cow;
use std::convert::{AsMut, AsRef};
use std::fmt::{Debug, Display, Formatter};

pub mod integer;
mod macros;
pub use integer::{AsInteger, Integer};
pub mod float;
pub use float::{AsFloat, Float};
pub mod unsigned_integer;
pub use unsigned_integer::{AsUnsignedInteger, UnsignedInteger};

use crate::{AsCell, Cell, AsNumber};

pub trait AsValue<'c> {
    fn as_value(&self) -> Value<'c>;
}

#[derive(Clone, PartialOrd, Ord, Default, PartialEq, Eq)]
pub enum Value<'c> {
    #[default]
    Nil,
    T,
    String(Cow<'c, str>),
    Symbol(&'static str),
    Byte(u8),
    UnsignedInteger(UnsignedInteger),
    Integer(Integer),
    Float(Float),
    List(Cell<'c>),
}
impl<'c> Value<'_> {
    pub fn nil() -> Value<'c> {
        Value::Nil
    }

    pub fn t() -> Value<'c> {
        Value::T
    }

    pub fn symbol<T: Display>(sym: T) -> Value<'c> {
        Value::Symbol(sym.to_string().leak())
    }

    pub fn string<T: Display>(value: T) -> Value<'c> {
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

    pub fn is_nil(&self) -> bool {
        if *self == Value::Nil {
            true
        } else {
            false
        }
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
                Value::UnsignedInteger(h) => format!("{}{}", h, UnsignedInteger::type_name()),
                Value::Integer(h) => format!("{}{}", h, Integer::type_name()),
                Value::Float(h) => format!("{}{}", h, Float::type_name()),
                Value::Symbol(h) => format!("{}", h),
                Value::List(h) => format!("{}", h),
            }
        )
    }
}

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
// impl<'c> From<&'static str> for Value<'c> {
//     fn from(value: &'static str) -> Value<'c> {
//         Value::Symbol(value)
//     }
// }
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

impl<'c> PartialEq<&mut Value<'c>> for Value<'c> {
    fn eq(&self, other: &&mut Value<'c>) -> bool {
        let other = &**other;
        self == other
    }
}

impl<'c> AsCell<'c> for Value<'c> {
    fn as_cell(&self) -> Cell<'c> {
        match self {
            Value::Nil => Cell::nil(),
            Value::T => Cell::from(Value::T),
            Value::String(value) => Cell::from(Value::string(value)),
            Value::Symbol(value) => Cell::from(Value::symbol(value)),
            Value::Byte(value) => Cell::from(Value::byte(*value)),
            Value::UnsignedInteger(value) => Cell::from(Value::unsigned_integer(*value)),
            Value::Integer(value) => Cell::from(Value::integer(*value)),
            Value::Float(value) => Cell::from(Value::float(*value)),
            Value::List(value) => Cell::from(Value::List(value.clone())),
        }
    }
}
