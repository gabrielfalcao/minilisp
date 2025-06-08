use std::borrow::Cow;
use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::str::FromStr;

use minilisp_util::{
    caller, dbg, extend_lifetime, try_result, unexpected, unwrap_result, with_caller,
};

use crate::Cell;

#[derive(Clone, PartialEq, PartialOrd, Default)]
pub enum Value<'c> {
    Byte(u8),
    Float(f64),
    Integer(i64),
    String(String),
    Symbol(Cow<'c, str>),
    // List(Cell<'c>),
    UnsignedInteger(u32),
    T,
    #[default]
    Nil,
}

impl<'c> Value<'c> {
    pub fn is_t(&self) -> bool {
        *self == Value::T
    }

    pub fn is_nil(&self) -> bool {
        *self == Value::Nil
    }

    pub fn is_float(&self) -> bool {
        self.as_float().is_some()
    }

    pub fn is_integer(&self) -> bool {
        self.as_integer().is_some()
    }

    pub fn is_unsigned_integer(&self) -> bool {
        self.as_unsigned_integer().is_some()
    }

    pub fn is_str<'c>(&self) -> bool {
        self.as_str().is_some()
    }

    pub fn as_float(&self) -> Option<f64> {
        if let Value::Float(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    pub fn as_integer(&self) -> Option<i64> {
        if let Value::Integer(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    pub fn as_unsigned_integer(&self) -> Option<u32> {
        if let Value::UnsignedInteger(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    pub fn as_str<'c>(&self) -> Option<&str> {
        if let Value::String(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn to_float(&self) -> f64 {
        if let Some(value) = self.as_float() {
            value
        } else {
            panic!("{:#?} is not a float", self);
        }
    }

    pub fn to_integer(&self) -> i64 {
        if let Some(value) = self.as_integer() {
            value
        } else {
            panic!("{:#?} is not a integer", self);
        }
    }

    pub fn to_unsigned_integer(&self) -> u32 {
        if let Some(value) = self.as_unsigned_integer() {
            value
        } else {
            panic!("{:#?} is not a unsigned", self);
        }
    }

    pub fn to_str(&self) -> &str {
        if let Some(value) = self.as_str() {
            value
        } else {
            panic!("{:#?} is not a str", self);
        }
    }
}

impl<'c, 'c> Into<i64> for Value<'c> {
    fn into(self) -> i64 {
        self.to_integer()
    }
}

impl<'c, 'c> Into<u32> for Value<'c> {
    fn into(self) -> u32 {
        self.to_unsigned_integer()
    }
}

impl<'c, 'c> Into<f64> for Value<'c> {
    fn into(self) -> f64 {
        self.to_float()
    }
}

impl<'c> From<u8> for Value<'c> {
    fn from(value: u8) -> Value<'c> {
        Value::Byte(value)
    }
}

impl<'c, 'c> From<&str> for Value<'c> {
    fn from(v: &str) -> Value<'c> {
        Value::Symbol(Cow::from(v.to_string()))
    }
}
impl<'c, 'c> From<String> for Value<'c> {
    fn from(v: String) -> Value<'c> {
        Value::String(v)
    }
}
impl<'c, 'c> From<i64> for Value<'c> {
    fn from(v: i64) -> Value<'c> {
        Value::Integer(v)
    }
}

impl<'c> PartialEq<&Value<'c>> for Value<'c> {
    fn eq(&self, other: &&Value<'c>) -> bool {
        self.eq(*other)
    }
}

impl<'c> PartialOrd<&Value<'c>> for Value<'c> {
    fn partial_cmp(&self, other: &&Value<'c>) -> Option<Ordering> {
        self.partial_cmp(*other)
    }
}

impl<'c> Display for Value<'c> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::T => "T".to_string(),
                Value::Byte(value) => format!("0x{:02x}", value),
                Value::Float(value) => value.to_string(),
                Value::Integer(value) => value.to_string(),
                Value::String(value) => value.to_string(),
                Value::UnsignedInteger(value) => value.to_string(),
                Value::Symbol(value) => value.to_string(),
                Value::List(value) => value.to_string(),
                Value::Nil => "Nil".to_string(),
            }
        )
    }
}
impl<'c> Debug for Value<'c> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}::Value::{}",
            module_path!(),
            match self {
                Value::T => "T".to_string(),
                Value::Byte(value) => format!("Byte(0x{:02x})", value),
                Value::Float(value) => format!("Float({})", value),
                Value::Integer(value) => format!("Integer({})", value),
                Value::String(value) => format!("String({:#?})", value),
                Value::UnsignedInteger(value) => format!("UnsignedInteger({})", value),
                Value::Symbol(value) => format!("Symbol({})", value),
                Value::List(value) => format!("List({})", value),
                Value::Nil => "Nil".to_string(),
            }
        )
    }
}

impl<'c> Hash for Value<'c> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        format!("{:#?}", self).hash(state);
    }
}
