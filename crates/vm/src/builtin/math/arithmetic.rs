use std::collections::{BTreeMap, VecDeque}; //BinaryHeap;

use minilisp_parser::{Item, Value};
use minilisp_util::{dbg, try_result};

use crate::helpers::{unpack_float_items, unpack_integer_items, unpack_unsigned_integer_items};
use crate::{
    impl_arithmetic_operation, with_caller, Closure, Error, ErrorType, Result, VirtualMachine,
};

impl_arithmetic_operation!(+ add);
impl_arithmetic_operation!(- sub);
impl_arithmetic_operation!(* mul);
impl_arithmetic_operation!(/ div);

#[macro_export]
macro_rules! impl_arithmetic_operation {
    (
        $operator:tt
            $function_name:ident
    ) => {
        pub fn $function_name<'c>(
            closure: &mut Closure<'c>,
            list: VecDeque<Item<'c>>,
        ) -> Result<Item<'c>> {
            let argcount = list.len();
            if argcount < 2 {
                return Err(with_caller!(closure.runtime_error(
                    format!(
                        "{:#?} takes at least 2 arguments, got: {}",
                        stringify!($operator),
                        argcount
                    ),
                    None
                )));
            }
            match list.front() {
                Some(Item::Value(Value::UnsignedInteger(first_operand))) => {
                    let mut operands =
                        try_result!(unpack_unsigned_integer_items(list).map_err(|error| {
                            Error::with_previous_error(
                                format!("call to {:#?} function", stringify!($operator)),
                                ErrorType::RuntimeError,
                                Some(error),
                            )
                        }));

                    let first = operands.pop_front().unwrap();
                    Ok(Item::Value(Value::UnsignedInteger(operands.into_iter().fold(first, |lhs, rhs| lhs $operator rhs))))
                },
                Some(Item::Value(Value::Integer(first_operand))) => {
                    let mut operands = try_result!(unpack_integer_items(list).map_err(|error| {
                        Error::with_previous_error(
                            format!("call to {:#?} function", stringify!($operator)),
                            ErrorType::RuntimeError,
                            Some(error),
                        )
                    }));
                    let first = operands.pop_front().unwrap();
                    Ok(Item::Value(Value::Integer(operands.into_iter().fold(first, |lhs, rhs| lhs $operator rhs))))

                },
                Some(Item::Value(Value::Float(first_operand))) => {
                    let mut operands = try_result!(unpack_float_items(list).map_err(|error| {
                        Error::with_previous_error(
                            format!("call to {:#?} function", stringify!($operator)),
                            ErrorType::RuntimeError,
                            Some(error),
                        )
                    }));
                    let first = operands.pop_front().unwrap();
                    Ok(Item::Value(Value::Float(operands.into_iter().fold(first, |lhs, rhs| lhs $operator rhs))))

                },
                Some(Item::Symbol(sym)) => {
                    todo!("evaluate symbol: {:#?}", sym);
                },
                Some(item) => Err(with_caller!(closure.runtime_error(
                    format!(
                        "{:#?} called with non-numerical value: {:#?}",
                        stringify!($operator),
                        item
                    ),
                    None
                ))),
                _ => {
                    unreachable!()
                },
            }
        }
    };
}
