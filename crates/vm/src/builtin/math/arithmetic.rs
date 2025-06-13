use std::collections::{BTreeMap, VecDeque}; //BinaryHeap;

use minilisp_data_structures::{append, car, cdr, list, Cell, Value, AsInteger, AsUnsignedInteger, AsFloat};
use minilisp_util::{dbg, try_result};

use crate::helpers::{
    unpack_float_items, unpack_integer_items, unpack_unsigned_integer_items,
};
use crate::{
    impl_arithmetic_operation, runtime_error, with_caller, Error, ErrorType,
    Result, VirtualMachine,
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
            vm: &mut VirtualMachine<'c>,
            list: Value<'c>,
        ) -> Result<Value<'c>> {
            let argcount = list.len();
            if argcount < 2 {
                return Err(with_caller!(runtime_error(
                    format!(
                        "{:#?} takes at least 2 arguments, got: {}",
                        stringify!($operator),
                        argcount
                    ),
                    None
                )));
            }
            match &car(&list) {
                Value::UnsignedInteger(first_operand)=> {
                    let mut operands =
                        try_result!(unpack_unsigned_integer_items(vm, list).map_err(|error| {
                            Error::with_previous_error(
                                format!("call to {:#?} function", stringify!($operator)),
                                ErrorType::RuntimeError,
                                Some(error),
                            )
                        }));

                    let first = car(&operands).as_unsigned_integer();
                    dbg!(&first_operand, &first, operands);
                    Ok(Value::UnsignedInteger(cdr(&operands).into_iter().fold(first, |lhs, rhs| lhs.as_unsigned_integer() $operator rhs.as_unsigned_integer())))
                },
                Value::Integer(first_operand) => {
                    let mut operands = try_result!(unpack_integer_items(vm, list).map_err(|error| {
                        Error::with_previous_error(
                            format!("call to {:#?} function", stringify!($operator)),
                            ErrorType::RuntimeError,
                            Some(error),
                        )
                    }));
                    let first = car(&operands).as_integer();
                    dbg!(&first_operand, &first, operands);
                    Ok(Value::Integer(cdr(&operands).into_iter().fold(first, |lhs, rhs| lhs.as_integer() $operator rhs.as_integer())))
                },
                Value::Float(first_operand) => {
                    let mut operands = try_result!(unpack_float_items(vm, list).map_err(|error| {
                        Error::with_previous_error(
                            format!("call to {:#?} function", stringify!($operator)),
                            ErrorType::RuntimeError,
                            Some(error),
                        )
                    }));
                    let first = car(&operands).as_float();
                    dbg!(&first_operand, &first, operands);
                    Ok(Value::Float(cdr(&operands).into_iter().fold(first, |lhs, rhs| lhs.as_float() $operator rhs.as_float())))

                },
                Value::Symbol(sym) => {
                    todo!("evaluate symbol: {:#?}", sym);
                },
                value => Err(with_caller!(runtime_error(
                    format!(
                        "{:#?} called with non-numerical value: {:#?}",
                        stringify!($operator),
                        value
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
