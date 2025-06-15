use std::borrow::Cow;
use std::collections::BTreeMap;

use minilisp_data_structures as ds;
use minilisp_data_structures::{AsCell, Quotable, Value};
use minilisp_util::{dbg, try_result, vec_deque};
use unique_pointer::UniquePointer;

use crate::helpers::runtime_error;
use crate::{with_caller, Error, ErrorType, Result, VirtualMachine};

pub fn list<'c>(
    mut vm: UniquePointer<VirtualMachine<'c>>,
    list: Value<'c>,
) -> Result<Value<'c>> {
    dbg!(&list);
    Ok(ds::list(list))
}

pub fn cons<'c>(
    mut vm: UniquePointer<VirtualMachine<'c>>,
    list: Value<'c>,
) -> Result<Value<'c>> {
    let cell = ds::cons(ds::car(&list), &mut ds::cdr(&list).as_cell());
    Ok(if list.is_quoted() {
        Value::quoted_list(cell)
    } else {
        Value::list(cell)
    })
}
pub fn quote<'c>(
    mut vm: UniquePointer<VirtualMachine<'c>>,
    value: Value<'c>,
) -> Result<Value<'c>> {
    Ok(match &value {
        Value::List(_) => value.clone().quote(),
        Value::QuotedList(_) => value.clone().quote(),
        Value::Symbol(_) => value.clone().quote(),
        Value::QuotedSymbol(_) => value.clone().quote(),
        item =>
            return Err(runtime_error(
                format!(
                    "quote invoked with non-symbol and non-list: {:#?}",
                    item
                ),
                None,
            )),
    })
}
pub fn backquote<'c>(
    mut vm: UniquePointer<VirtualMachine<'c>>,
    list: Value<'c>,
) -> Result<Value<'c>> {
    Ok(list)
}

pub fn car<'c>(
    mut vm: UniquePointer<VirtualMachine<'c>>,
    list: Value<'c>,
) -> Result<Value<'c>> {
    dbg!(&list);
    Ok(ds::car(&ds::car(&list)))
}

pub fn cdr<'c>(
    mut vm: UniquePointer<VirtualMachine<'c>>,
    list: Value<'c>,
) -> Result<Value<'c>> {
    dbg!(&list);
    Ok(ds::cdr(&list))
}
