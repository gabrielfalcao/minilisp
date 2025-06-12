#![allow(unused)]
use std::fmt::{Debug, Display, Formatter};

use unique_pointer::UniquePointer;

use crate::{AsCell, AsValue, Cell, Quotable, Value};

pub trait AsList<'c>: Quotable {
    fn as_list(&self) -> List<'c>;
}
pub trait ListIterator<'c, T: AsCell<'c>>: IntoIterator<Item = T> {
    fn list_iter(&self) -> Cell<'c>;
}

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq)]
pub enum List<'c> {
    Empty(bool),
    Linked(Cell<'c>, bool),
}
impl<'c> Default for List<'c> {
    fn default() -> List<'c> {
        List::Empty(false) // empty non-quoted list
    }
}

impl<'c> List<'c> {
    pub fn is_nil(&self) -> bool {
        if let List::Empty(quoted) = self {
            !*quoted
        } else {
            false
        }
    }

    pub fn nil() -> List<'c> {
        List::default()
    }

    pub fn empty(quoted: bool) -> List<'c> {
        List::quoted::<Value>(None, quoted)
    }

    pub fn new<T: AsCell<'c>>(item: T) -> List<'c> {
        List::quoted(Some(item), false)
    }

    pub fn values(&self) -> Vec<Value<'c>> {
        match self {
            List::Empty(_) => Vec::new(),
            List::Linked(cell, _) => cell.values(),
        }
    }

    pub fn head(&self) -> Value<'c> {
        match self.clone() {
            List::Empty(quoted) =>
                Cell::quoted(Option::<Value<'c>>::None, quoted).as_value(),
            List::Linked(cell, quoted) => {
                let mut cell = Cell::from(cell.head().unwrap_or_default());
                if quoted {
                    cell.quote()
                } else {
                    cell
                }.as_value()
            },
        }
    }

    pub fn tail(&self) -> Cell<'c> {
        match self.clone() {
            List::Empty(quoted) =>
                Cell::quoted(Option::<Value<'c>>::None, quoted),
            List::Linked(cell, quoted) => {
                let mut cell =
                    cell.tail().map(Clone::clone).unwrap_or_default();
                if quoted {
                    cell.quote()
                } else {
                    cell
                }
            },
        }
    }

    pub fn quoted<T: AsCell<'c>>(item: Option<T>, quoted: bool) -> List<'c> {
        if let Some(item) = item {
            let cell = item.as_cell();
            if cell.is_nil() {
                List::Empty(quoted)
            } else {
                List::Linked(item.as_cell(), quoted)
            }
        } else {
            List::Empty(quoted)
        }
    }

    pub fn quote(&self) -> List<'c> {
        match self.clone() {
            List::Empty(_) => List::<'c>::Empty(true),
            List::Linked(cell, _) => List::<'c>::Linked(cell.clone(), true),
        }
    }
}
impl<'c> Quotable for List<'c> {
    fn is_quoted(&self) -> bool {
        match self {
            List::Empty(quoted) => *quoted,
            List::Linked(_, quoted) => *quoted,
        }
    }
}
impl Display for List<'_> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            [match self {
                List::Empty(quoted) => {
                    if *quoted {
                        "'()".to_string()
                    } else {
                        "nil".to_string()
                    }
                },
                List::Linked(cell, quoted) => {
                    if *quoted {
                        format!("'({})", cell)
                    } else {
                        format!("({})", cell)
                    }
                },
            }]
            .join("")
        )
    }
}
impl Debug for List<'_> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl<'c> AsList<'c> for List<'c> {
    fn as_list(&self) -> List<'c> {
        self.clone()
    }
}

impl<'c> AsValue<'c> for List<'c> {
    fn as_value(&self) -> Value<'c> {
        match self {
            List::Empty(quoted) => {
                let quoted = *quoted;
                if quoted {
                    Value::EmptyQuotedList
                } else {
                    Value::EmptyList
                }
            },
            List::Linked(cell, quoted) => {
                let quoted = *quoted;
                if quoted {
                    Value::QuotedList(cell.as_list())
                } else {
                    Value::List(cell.as_list())
                }
            },
        }
    }
}

impl<'c> AsCell<'c> for List<'c> {
    fn as_cell(&self) -> Cell<'c> {
        match self {
            List::Empty(quoted) => Cell::quoted::<Value<'c>>(None, *quoted),
            List::Linked(list, quoted) => {
                let mut cell = Cell::nil();
                for item in list.clone() {
                    cell.add(&Cell::from(item));
                }
                Cell::quoted(Some(cell), *quoted)
            },
        }
    }
}
impl<'c> AsCell<'c> for &List<'c> {
    fn as_cell(&self) -> Cell<'c> {
        let list = UniquePointer::read_only(*self).read();
        list.as_cell()
    }
}

impl<'c> AsList<'c> for Value<'c> {
    fn as_list(&self) -> List<'c> {
        match self {
            Value::List(h) => {
                // normalizes quoting
                if h.is_nil() {
                    List::Empty(h.is_quoted())
                } else {
                    if h.is_quoted() {
                        List::Linked(h.as_cell(), true)
                    } else {
                        List::Linked(h.as_cell(), false)
                    }
                }
            },
            Value::QuotedList(h) => {
                // normalizes quoting

                if h.is_nil() {
                    List::Empty(h.is_quoted())
                } else {
                    if h.is_quoted() {
                        List::Linked(h.as_cell(), true)
                    } else {
                        List::Linked(h.as_cell(), false)
                    }
                }
            },
            Value::QuotedSymbol(h) => {
                // normalizes quoting
                let mut cell = Cell::from(h.clone());
                List::Linked(
                    if h.is_quoted() {
                        cell.quote()
                    } else {
                        cell
                    },
                    h.is_quoted(),
                )
            },
            Value::Symbol(h) => {
                // normalizes quoting
                let mut cell = Cell::from(h);
                List::Linked(
                    if h.is_quoted() {
                        cell.quote()
                    } else {
                        cell
                    },
                    h.is_quoted(),
                )
            },
            Value::Nil => List::nil(),
            _ => List::Linked(Cell::from(self.clone()), self.is_quoted()),
        }
    }
}
impl<'c> From<Cell<'c>> for List<'c> {
    fn from(cell: Cell<'c>) -> List<'c> {
        cell.as_list()
    }
}
