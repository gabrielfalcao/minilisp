#![allow(unused)]
use crate::{AsCell, AsValue, Cell, ListIterator, Quotable, Symbol, Value};
use minilisp_util::dbg;
pub fn cons<'c, T: AsCell<'c>>(head: T, tail: &mut Cell<'c>) -> Cell<'c> {
    let mut head = head.as_cell();
    head.add(tail);
    head
}
pub fn append<'c, T: ListIterator<'c, Value<'c>>>(list: T) -> Value<'c> {
    let mut items = Cell::nil();
    for value in list {
        match &value {
            Value::List(h) | Value::QuotedList(h) => {
                for item in h.clone().into_iter() {
                    items.add(&Cell::from(item));
                }
            },
            Value::EmptyList |
            Value::EmptyQuotedList |
            Value::Nil => {}
            value => {
                items.add(&Cell::from(value));
            }
        }
    }
    Value::list(items)
}
pub fn makelist<'c>(value: Value<'c>, count: usize) -> Value<'c> {
    (0..count).map(|_|value.clone()).collect()
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
    // dbg!(&list);
    for item in list.into_iter() {
        cell.push_value(item);
    }
    Value::List(cell)
}

pub fn setcar<'c>(cell: &Cell<'c>, sym: &Symbol, value: &Value) {}
pub fn setcdr<'c>(cell: &Cell<'c>, value: &Value) {}
