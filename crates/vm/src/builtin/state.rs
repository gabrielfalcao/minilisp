use std::borrow::Cow;
use std::collections::{BTreeMap, VecDeque}; //BinaryHeap;

use minilisp_data_structures::{car, cdr, list, append, Value, Cell, AsSymbol};
use minilisp_util::{dbg, try_result};

use crate::helpers::{
    runtime_error, unpack_float_items, unpack_integer_items, unpack_unsigned_integer_items,
};
use crate::{with_caller, Error, ErrorType, Result, VirtualMachine};

pub fn setq<'c>(vm: &mut VirtualMachine<'c>, list: Value<'c>) -> Result<Value<'c>> {
    // let list = vm.eval_list_as_items(list)?;
    if list.len() % 2 != 0 {
        return Err(runtime_error(
            format!("odd number of arguments ({}) in setq: {:#?}", list.len(), list),
            None,
        ));
    }
    let head = car(&list);
    if !head.is_symbol() {
        return Err(runtime_error(format!("setq invoked with non-symbol: {:#?}", head), None));
    }
    Ok(vm.setq(head.as_symbol(), cdr(&list))?)
}
