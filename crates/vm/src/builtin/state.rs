use std::borrow::Cow;
use std::collections::{BTreeMap, VecDeque}; //BinaryHeap;

use minilisp_parser::{Item, Value};
use minilisp_util::{dbg, try_result};

use crate::helpers::{
    runtime_error, unpack_float_items, unpack_integer_items, unpack_unsigned_integer_items,
};
use crate::{with_caller, Error, ErrorType, Result, VirtualMachine};

pub fn setq<'c>(vm: &mut VirtualMachine<'c>, list: VecDeque<Item<'c>>) -> Result<Item<'c>> {
    if list.is_empty() {
        return Ok(Item::Value(Value::Nil));
    }
    // let list = vm.eval_list_as_items(list)?;
    if list.len() % 2 != 0 {
        return Err(runtime_error(
            format!("odd number of arguments ({}) in setq: {:#?}", list.len(), list),
            None,
        ));
    }
    let mut list = list.clone();
    let car = list.pop_front().unwrap().clone();
    if let Some(symbol) = car.as_symbol() {
        let cdr = list.pop_front().unwrap().clone();
        Ok(vm.setq(symbol.to_string(), cdr.clone())?)
    } else {
        Err(runtime_error(format!("setq invoked with non-symbol car {:#?}", car), None))
    }
}
