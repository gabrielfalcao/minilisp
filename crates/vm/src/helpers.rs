use std::collections::{BTreeMap, VecDeque}; //BinaryHeap;

use minilisp_parser::{Item, Value};
use minilisp_util::{dbg, try_result};

use crate::{impl_unpack_values, with_caller, Error, ErrorType, Result, VirtualMachine};

impl_unpack_values!(unpack_unsigned_integer_items, Value::UnsignedInteger, u32);
impl_unpack_values!(unpack_integer_items, Value::Integer, i64);
impl_unpack_values!(unpack_float_items, Value::Float, f64);

pub fn runtime_error(message: String, previous: Option<Error>) -> Error {
    with_caller!(Error::with_previous_error(message, ErrorType::RuntimeError, previous))
}

#[macro_export]
macro_rules! impl_unpack_values {
    ($function_name:ident,Value:: $value_variant:ident, $type:ty) => {
        pub fn $function_name<'c>(
            vm: &mut VirtualMachine,
            list: VecDeque<Item<'c>>,
        ) -> Result<VecDeque<$type>> {
            let vm = unsafe {std::mem::transmute::<&mut VirtualMachine, &'c mut VirtualMachine>(vm)};
            let mut items = VecDeque::<$type>::new();
            let mut errors = VecDeque::<Error>::new();
            for (index, item) in list.into_iter().enumerate() {
                match item.clone() {
                    Item::Value(Value::$value_variant(value)) => {
                        items.push_back(value);
                    },
                    Item::Value(_) => {
                        errors.push_front(with_caller!(Error::with_previous_error(
                            format!(
                                "{}th value is not Value::{} but {:#?}",
                                index,
                                stringify!($value_variant),
                                item
                            ),
                            ErrorType::RuntimeError,
                            errors.front().map(Clone::clone)
                        )));
                    },
                    Item::Symbol(sym) => {
                        match vm.eval_symbol(&sym)? {
                            Value::$value_variant(value) => {
                                items.push_back(value);
                            },
                            _ => {
                                errors.push_front(with_caller!(Error::with_previous_error(
                                    format!(
                                        "{}th value is not Value::{} but {:#?}",
                                        index,
                                        stringify!($value_variant),
                                        item
                                    ),
                                    ErrorType::RuntimeError,
                                    errors.front().map(Clone::clone)
                                )));
                            },
                        }
                        // errors.push_front(with_caller!(Error::with_previous_error(
                        //     format!("{}th item is {:#?}", index, item),
                        //     ErrorType::RuntimeError,
                        //     errors.front().map(Clone::clone)
                        // )));
                    },
                    Item::List(list) => match vm.eval_list(list.clone())? {
                        Value::$value_variant(value) => {
                            items.push_back(value);
                        },
                        _ => {
                            errors.push_front(with_caller!(Error::with_previous_error(
                                format!(
                                    "{}th value is not Value::{} but {:#?}",
                                    index,
                                    stringify!($value_variant),
                                    item
                                ),
                                ErrorType::RuntimeError,
                                errors.front().map(Clone::clone)
                            )));
                        },
                    },
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
    };
}
