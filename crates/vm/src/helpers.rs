use std::collections::{BTreeMap, VecDeque}; //BinaryHeap;

use minilisp_parser::{Item, Value};
use minilisp_util::{dbg, try_result};

use crate::{with_caller, Error, ErrorType, Result, VirtualMachine};

pub fn unpack_unsigned_integer_items<'c>(list: VecDeque<Item<'c>>) -> Result<VecDeque<u64>> {
    let mut items = VecDeque::<u64>::new();
    let mut errors = VecDeque::<Error>::new();
    for (index, item) in list.into_iter().enumerate() {
        if let Item::Value(Value::UnsignedInteger(value)) = item {
            items.push_back(value);
        } else {
            errors.push_front(with_caller!(Error::with_previous_error(
                format!("{}th item is {:#?}", index, item),
                ErrorType::RuntimeError,
                errors.front().map(Clone::clone)
            )));
        }
    }
    if errors.is_empty() {
        Ok(items)
    } else {
        Err(with_caller!(Error::with_previous_error(
            format!("expected all list items to be unsigned integer"),
            ErrorType::RuntimeError,
            errors.front().map(Clone::clone)
        )))
    }
}

pub fn unpack_integer_items<'c>(list: VecDeque<Item<'c>>) -> Result<VecDeque<i64>> {
    let mut items = VecDeque::<i64>::new();
    let mut errors = VecDeque::<Error>::new();
    for (index, item) in list.into_iter().enumerate() {
        if let Item::Value(Value::Integer(value)) = item {
            items.push_back(value);
        } else {
            errors.push_front(with_caller!(Error::with_previous_error(
                format!("{}th item is {:#?}", index, item),
                ErrorType::RuntimeError,
                errors.front().map(Clone::clone)
            )));
        }
    }
    if errors.is_empty() {
        Ok(items)
    } else {
        Err(with_caller!(Error::with_previous_error(
            format!("expected all list items to be integer"),
            ErrorType::RuntimeError,
            errors.front().map(Clone::clone)
        )))
    }
}

pub fn unpack_float_items<'c>(list: VecDeque<Item<'c>>) -> Result<VecDeque<f64>> {
    let mut items = VecDeque::<f64>::new();
    let mut errors = VecDeque::<Error>::new();
    for (index, item) in list.into_iter().enumerate() {
        if let Item::Value(Value::Float(value)) = item {
            items.push_back(value);
        } else {
            errors.push_front(with_caller!(Error::with_previous_error(
                format!("{}th item is {:#?}", index, item),
                ErrorType::RuntimeError,
                errors.front().map(Clone::clone)
            )));
        }
    }
    if errors.is_empty() {
        Ok(items)
    } else {
        Err(with_caller!(Error::with_previous_error(
            format!("expected all list items to be float"),
            ErrorType::RuntimeError,
            errors.front().map(Clone::clone)
        )))
    }
}

pub fn runtime_error(message: String, previous: Option<Error>) -> Error {
    with_caller!(Error::with_previous_error(message, ErrorType::RuntimeError, previous))
}
