#![allow(unused)]
use minilisp_data_structures::Value;

use crate::{Result, VirtualMachine};
use unique_pointer::UniquePointer;

pub fn t<'c>(
    mut vm: UniquePointer<VirtualMachine<'c>>,
    list: Value<'c>,
) -> Result<Value<'c>> {
    Ok(Value::T)
}
