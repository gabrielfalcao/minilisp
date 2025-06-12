#![allow(unused)]
use std::fmt::{Debug, Display, Formatter};

use unique_pointer::UniquePointer;

use crate::{AsValue, Quotable, Value};

pub trait AsSymbol<'c> {
    fn as_symbol(&self) -> Symbol<'c>;
}
//// impl<'c> AsSymbol for str {
////     fn as_symbol(&self) -> Symbol<'c> {
////         Symbol::new(self)
////     }
//// }
//// impl<'c> AsSymbol for String {
////     fn as_symbol(&self) -> Symbol<'c> {
////         Symbol::new(self)
////     }
//// }
//// impl<'c> AsSymbol for Cow<'c, str> {
////     fn as_symbol(&self) -> Symbol<'c> {
////         Symbol::new(&self)
////     }
//// }

#[derive(Clone, PartialOrd, Ord, Default, PartialEq, Eq)]
pub struct Symbol<'c> {
    sym: &'c str,
    quoted: bool,
}
impl<'c> Symbol<'c> {
    pub fn new<T: ToString>(sym: T) -> Symbol<'c> {
        Symbol::quoted(sym, false)
    }

    pub fn quoted<T: ToString>(sym: T, quoted: bool) -> Symbol<'c> {
        Symbol {
            sym: sym.to_string().leak(),
            quoted,
        }
    }

    pub fn quote(&self) -> Symbol<'c> {
        let mut symbol = self.clone();
        symbol.quoted = true;
        symbol
    }

    pub fn unquote(&self) -> Symbol<'c> {
        let mut symbol = self.clone();
        symbol.quoted = false;
        symbol
    }

    pub fn is_quoted(&self) -> bool {
        self.quoted
    }
}

impl Display for Symbol<'_> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            [
                if self.quoted {
                    "'".to_string()
                } else {
                    String::new()
                },
                self.sym.to_string()
            ]
            .join("")
        )
    }
}
impl Debug for Symbol<'_> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "#[{}]#", self.to_string())
    }
}

impl<'c> From<&'c str> for Symbol<'c> {
    fn from(symbol: &'c str) -> Symbol<'c> {
        Symbol::new(symbol)
    }
}

impl<'c> From<String> for Symbol<'c> {
    fn from(symbol: String) -> Symbol<'c> {
        Symbol::new(&symbol)
    }
}

impl<'c> AsSymbol<'c> for Symbol<'c> {
    fn as_symbol(&self) -> Symbol<'c> {
        self.clone()
    }
}
impl<'c> AsSymbol<'c> for &Symbol<'c> {
    fn as_symbol(&self) -> Symbol<'c> {
        UniquePointer::read_only(*self).read()
    }
}

impl<'c> AsSymbol<'c> for String {
    fn as_symbol(&self) -> Symbol<'c> {
        Symbol::new(self)
    }
}

impl<'c> AsSymbol<'c> for &str {
    fn as_symbol(&self) -> Symbol<'c> {
        Symbol::new(*self)
    }
}

impl<'c> Quotable for Symbol<'c> {
    fn is_quoted(&self) -> bool {
        self.quoted
    }
}
impl<'c> AsValue<'c> for Symbol<'c> {
    fn as_value(&self) -> Value<'c> {
        Value::Symbol(self.clone())
    }
}
// // impl<'c> AsRef<Symbol<'c>> for Symbol<'c> {
// //     fn as_ref(&self) -> &Symbol<'c> {
// //         self
// //     }
// // }
// //
// // impl<'c, T: AsRef<Symbol<'c>>> AsSymbol<'c> for T {
// //     fn as_symbol(&self) -> Symbol<'c> {
// //         self.as_ref().clone()
// //     }
// // }
