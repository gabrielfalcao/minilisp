use std::borrow::Cow;
use std::collections::{BTreeMap, VecDeque}; //BinaryHeap;

use minilisp_parser::{Item, Value};
use minilisp_util::{dbg, try_result};

use crate::helpers::{unpack_float_items, unpack_integer_items, unpack_unsigned_integer_items};
use crate::{with_caller, Error, ErrorType, Result, VirtualMachine};

pub fn print<'c>(vm: &mut VirtualMachine<'c>, list: VecDeque<Item<'c>>) -> Result<Item<'c>> {
    // let mut parts = Vec::new();
    // for item in list.into_iter() {
    //     parts.push(match item {
    //         Item::Value(value) => value,
    //         Item::List(list) => {
    //             try_result!(vm.eval_list(list))
    //         },
    //         Item::Symbol(sym) => {
    //             try_result!(vm.eval_symbol(&sym))
    //         },
    //     }.to_string());
    // }
    // let string = parts.join(" ");
    Ok(Item::Value(try_result!(vm.eval_list(list))))
}
