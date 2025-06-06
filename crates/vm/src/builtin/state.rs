use std::borrow::Cow;
use std::collections::{BTreeMap, VecDeque}; //BinaryHeap;

use minilisp_parser::{Item, Value};
use minilisp_util::{dbg, try_result};

use crate::helpers::{
    runtime_error, unpack_float_items, unpack_integer_items, unpack_unsigned_integer_items,
};
use crate::{with_caller, Error, ErrorType, Result, VirtualMachine};

pub fn setq<'c>(closure: &mut VirtualMachine<'c>, list: VecDeque<Item<'c>>) -> Result<Item<'c>> {
    if list.is_empty() {
        return Ok(Item::Value(Value::Nil));
    }
    if list.len() % 2 != 0 {
        return Err(runtime_error(format!("odd number of arguments in setq: {}", list.len()), None));
    }
    let mut list = list.clone();
    let cdr = loop {
        let car = list.pop_front().unwrap().clone();
        if let Some(symbol) = car.as_symbol() {
            let cdr = list.pop_front().unwrap().clone();
            closure.setq(symbol.to_string(), cdr.clone());
            if list.is_empty() {
                break cdr
            }
        } else {
            return Err(runtime_error(format!("setq invoked with non-symbol car {:#?}", car), None));
        }
    };

    Ok(cdr)
}
