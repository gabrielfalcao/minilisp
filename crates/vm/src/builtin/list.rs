use std::borrow::Cow;
use std::collections::{BTreeMap, VecDeque}; //BinaryHeap;

use minilisp_parser::{Item, Value};
use minilisp_util::{dbg, try_result};

use crate::helpers::{unpack_float_items, unpack_integer_items, unpack_unsigned_integer_items};
use crate::{with_caller, Error, ErrorType, Result, VirtualMachine};

pub fn list<'c>(closure: &mut VirtualMachine<'c>, list: VecDeque<Item<'c>>) -> Result<Item<'c>> {
    if list.is_empty() {
        return Ok(Item::Value(Value::Nil))
    }
    let mut list = list.clone();
    let car = list.pop_front().unwrap();
    Ok(Item::Value(Value::Nil))

}

pub fn cons<'c>(closure: &mut VirtualMachine<'c>, list: VecDeque<Item<'c>>) -> Result<Item<'c>> {
    if list.is_empty() {
        return Ok(Item::Value(Value::Nil))
    }
    let mut list = list.clone();
    let car = list.pop_front().unwrap();
    Ok(Item::Value(Value::Nil))
}

pub fn car<'c>(closure: &mut VirtualMachine<'c>, list: VecDeque<Item<'c>>) -> Result<Item<'c>> {
    if list.is_empty() {
        return Ok(Item::Value(Value::Nil))
    }
    let mut list = list.clone();
    let car = list.pop_front().unwrap().clone();
    Ok(car)
}

pub fn cdr<'c>(closure: &mut VirtualMachine<'c>, list: VecDeque<Item<'c>>) -> Result<Item<'c>> {
    if list.is_empty() {
        return Ok(Item::Value(Value::Nil))
    }
    let mut list = list.clone();
    list.pop_front().unwrap();
    Ok(Item::List(list))
}
