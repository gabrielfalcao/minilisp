#![allow(unused)]
use crate::{AsCell, AsValue, Cell, Symbol, Value, List, AsList, ListIterator};

pub fn cons<'c, H: AsCell<'c>>(head: H, tail: &mut Cell<'c>) -> Cell<'c> {
    let mut head = head.as_cell();
    head.add(tail);
    head
}
pub fn append<'c, H: AsCell<'c>, I: Iterator<Item = H>>(
    _cell: &mut Cell<'c>,
    _iter: I,
) -> Cell<'c> {
    Cell::nil()
}
pub fn makelist<'c, V: AsValue<'c>>(value: V, count: usize) -> Cell<'c> {
    Cell::nil()
}
pub fn car<'c, H: AsCell<'c>>(cell: H) -> Value<'c> {
    let cell = cell.as_cell();
    if let Some(head) = cell.head() {
        head
    } else {
        Value::nil()
    }
}

pub fn cdr<'c, H: AsCell<'c>>(item: H) -> List<'c> {
    let cell = item.as_cell();
    if let Some(tail) = cell.tail() {
        tail.clone().as_list()
    } else {
        eprintln!("\nOOPS NO TAIL\n");
        eprintln!("\nOOPS NO TAIL\n");
        eprintln!("\nOOPS NO TAIL\n");
        Cell::nil().as_list()
    }
}
pub fn list<'c, C: AsCell<'c>, T: ListIterator<'c, C>>(list: T) -> List<'c> {
    let mut cell = Cell::nil();
    for item in list.into_iter() {
        cell.add(&item.as_cell());
    }
    List::Linked(cell, false)
}

pub fn setcar<'c>(cell: &Cell<'c>, sym: &Symbol, value: &Value) {}
pub fn setcdr<'c>(cell: &Cell<'c>, value: &Value) {}
