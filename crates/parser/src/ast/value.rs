use std::borrow::Cow;
use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::str::FromStr;

use minilisp_util::{
    caller, dbg, extend_lifetime, string_to_str, try_result, unexpected, unwrap_result, with_caller,
};
use pest::iterators::Pair;

use crate::Rule;
#[derive(Clone, PartialEq, PartialOrd, Default)]
pub enum Value<'a> {
    Float(f64),
    Integer(i64),
    String(&'a str),
    // String(Cow<'a, str>),
    UnsignedInteger(u64),
    T,
    #[default]
    Nil,
}

impl<'a> Value<'a> {
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

    pub fn as_unsigned_integer(&self) -> Option<u64> {
        if let Value::UnsignedInteger(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    pub fn as_str<'c>(&self) -> Option<&'c str> {
        if let Value::String(value) = self {
            Some(string_to_str!(&value, 'c))
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

    pub fn to_unsigned_integer(&self) -> u64 {
        if let Some(value) = self.as_unsigned_integer() {
            value
        } else {
            panic!("{:#?} is not a unsigned", self);
        }
    }

    pub fn to_str<'c>(&self) -> &'c str {
        if let Some(value) = self.as_str() {
            value
        } else {
            panic!("{:#?} is not a str", self);
        }
    }
}
impl<'a> Value<'a> {
    pub fn from_pair(pair: &mut Pair<Rule>) -> Value<'a> {
        match pair.as_rule() {
            Rule::float => Value::Float(f64::from_str(pair.as_span().as_str()).expect("float")),
            Rule::integer =>
                Value::Integer(i64::from_str(pair.as_span().as_str()).expect("integer")),
            Rule::string => Value::String(string_to_str!(&pair.as_span().as_str(), 'a)),
            Rule::t => Value::T,
            Rule::unsigned => Value::UnsignedInteger(
                u64::from_str(pair.as_span().as_str()).expect("unsigned integer"),
            ),
            Rule::value => Value::from_pair(&mut pair.clone().into_inner().next().expect("value")),
            Rule::nil => Value::Nil,
            _ => unexpected!(pair),
        }
    }
}

impl<'a, 'c> Into<i64> for Value<'a> {
    fn into(self) -> i64 {
        self.to_integer()
    }
}

impl<'a, 'c> Into<u64> for Value<'a> {
    fn into(self) -> u64 {
        self.to_unsigned_integer()
    }
}

impl<'a, 'c> Into<f64> for Value<'a> {
    fn into(self) -> f64 {
        self.to_float()
    }
}

impl<'a> Into<&'a str> for Value<'a> {
    fn into(self) -> &'a str {
        self.to_str()
    }
}

impl<'a> PartialEq<&Value<'a>> for Value<'a> {
    fn eq(&self, other: &&Value<'a>) -> bool {
        self.eq(*other)
    }
}

impl<'a> PartialOrd<&Value<'a>> for Value<'a> {
    fn partial_cmp(&self, other: &&Value<'a>) -> Option<Ordering> {
        self.partial_cmp(*other)
    }
}

impl<'a> Display for Value<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::Float(value) => value.to_string(),
                Value::Integer(value) => value.to_string(),
                Value::String(value) => format!("{:#?}", value),
                Value::UnsignedInteger(value) => value.to_string(),
                Value::Nil => "nil".to_string(),
                Value::T => "t".to_string(),
            }
        )
    }
}
impl<'a> Debug for Value<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}::Value::{}",
            module_path!(),
            match self {
                Value::Float(value) => format!("Float({:#?})", value),
                Value::Integer(value) => format!("Integer({:#?})", value),
                Value::String(value) => format!("String({:#?})", value),
                Value::UnsignedInteger(value) => format!("UnsignedInteger({:#?})", value),
                Value::Nil => "Nil".to_string(),
                Value::T => "T".to_string(),
            }
        )
    }
}

impl<'a> Hash for Value<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        format!("{:#?}", self).hash(state);
    }
}
