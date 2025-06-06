use std::collections::{BTreeMap, VecDeque}; //BinaryHeap;

use minilisp_parser::{Item, Value};
use minilisp_util::{dbg, try_result};

use crate::helpers::{unpack_float_items, unpack_integer_items, unpack_unsigned_integer_items};
use crate::{string_to_str, with_caller, Closure, Error, ErrorType, Result, VirtualMachine};

pub fn print<'c>(closure: &mut Closure<'c>, list: VecDeque<Item<'c>>) -> Result<Item<'c>> {
    let string = format!(
        "{}",
        list.into_iter()
            .map(|item| match item {
                Item::Value(value) => value.to_string(),
                Item::List(list) => {
                    format!("{:#?}", list)
                },
                Item::Symbol(sym) => {
                    todo!("evaluate symbol: {:#?}", sym);
                },
            })
            .collect::<Vec<String>>()
            .join(" ")
    );
    Ok(Item::Value(Value::String(string_to_str!(&string.as_str(), 'c))))
}
