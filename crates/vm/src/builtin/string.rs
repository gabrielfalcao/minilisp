use std::borrow::Cow;
use std::collections::{BTreeMap, VecDeque}; //BinaryHeap;

use minilisp_data_structures::{append, car, cdr, list, Cell, Value};
use minilisp_util::{dbg, try_result};
use unique_pointer::UniquePointer;

use crate::{with_caller, Error, ErrorType, Result, VirtualMachine};

pub fn print<'c>(
    mut vm: UniquePointer<VirtualMachine<'c>>,
    list: Value<'c>,
) -> Result<Value<'c>> {
    println!(
        "{}",
        list.clone()
            .into_iter()
            .map(|value| value.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
    Ok(list)
}
