#![allow(unused)]
use crate::{AsCell, AsValue, Cell, ListIterator, Quotable, Symbol, Value};

pub fn cons<'c, T: AsCell<'c>>(head: T, tail: &mut Cell<'c>) -> Cell<'c> {
    let mut head = head.as_cell();
    head.add(tail);
    head
}
pub fn append<'c, T: AsCell<'c>, I: Iterator<Item = T>>(
    _cell: &mut Cell<'c>,
    _iter: I,
) -> Cell<'c> {
    Cell::nil()
}
pub fn makelist<'c, V: AsValue<'c>>(value: V, count: usize) -> Cell<'c> {
    Cell::nil()
}
pub fn car<'c>(value: &Value<'c>) -> Value<'c> {
    match value {
        Value::List(cell) | Value::QuotedList(cell) =>
            cell.head().unwrap_or_default(),
        _ => Value::Nil,
    }
}

pub fn cdr<'c>(item: &Value<'c>) -> Value<'c> {
    match item {
        Value::List(ref h) | Value::QuotedList(ref h) => h
            .tail()
            .map(|cell| Value::list(cell.clone()))
            .unwrap_or_default(),
        _ => Value::Nil,
    }
}
pub fn list<'c, T: ListIterator<'c, Value<'c>>>(list: T) -> Value<'c> {
    let mut cell = Cell::nil();
    for item in list.into_iter() {
        cell.add(&Cell::from(item));
    }
    Value::List(cell)
}

pub fn setcar<'c>(cell: &Cell<'c>, sym: &Symbol, value: &Value) {}
pub fn setcdr<'c>(cell: &Cell<'c>, value: &Value) {}
