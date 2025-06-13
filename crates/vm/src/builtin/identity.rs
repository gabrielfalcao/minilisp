use minilisp_data_structures::Value;

use crate::{Result, VirtualMachine};

pub fn t<'c>(
    _vm: &mut VirtualMachine<'c>,
    _list: Value<'c>,
) -> Result<Value<'c>> {
    Ok(Value::T)
}
