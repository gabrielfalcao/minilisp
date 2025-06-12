#![allow(unused)]
use crate::{AsCell, AsValue, Cell, Symbol, Value, ListIterator, Quotable};

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
pub fn car<'c, T: AsCell<'c>>(cell: T) -> Value<'c> {
    let cell = cell.as_cell();
    cell.unwrap_value()
}

pub fn cdr<'c, T: AsCell<'c>>(item: T) -> Value<'c> {
    let cell = item.as_cell();
    if let Some(tail) = cell.tail() {
        tail.clone().as_value()
    } else {
        Cell::nil().as_value()
    }
}
pub fn list<'c, C: AsCell<'c>, T: ListIterator<'c, C>>(list: T) -> Value<'c> {
    let mut cell = Cell::nil();
    for item in list.into_iter() {
        cell.add(&item.as_cell());
    }
    Value::List(cell)
}

pub fn setcar<'c>(cell: &Cell<'c>, sym: &Symbol, value: &Value) {}
pub fn setcdr<'c>(cell: &Cell<'c>, value: &Value) {}
