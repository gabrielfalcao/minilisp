use std::borrow::Cow;
use std::collections::VecDeque;
use std::fmt::Display;

use crate::Value;

#[derive(Clone, PartialEq, PartialOrd, Debug, Hash)]
pub enum Item<'a> {
    List(VecDeque<Item<'a>>),
    Symbol(Cow<'a, str>),
    Value(Value<'a>),
}

impl<'a> Item<'a> {
    pub fn as_sexpr(&self) -> Option<VecDeque<Item<'a>>> {
        if let Item::List(items) = self {
            Some(items.clone())
        } else {
            None
        }
    }

    pub fn as_symbol<'c>(&self) -> Option<String> {
        if let Item::Symbol(symbol) = self {
            Some(symbol.to_string())
        } else {
            None
        }
    }

    pub fn as_value<'c>(&self) -> Option<String> {
        if let Item::Symbol(symbol) = self {
            Some(symbol.to_string())
        } else {
            None
        }
    }
}
impl<'a> Item<'a> {
    pub fn symbol<T: Display>(sym: T) -> Item<'a> {
        Item::Symbol(Cow::from(sym.to_string()))
    }
}
