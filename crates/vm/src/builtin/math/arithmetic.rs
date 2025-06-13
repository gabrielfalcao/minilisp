use std::collections::{BTreeMap, VecDeque}; //BinaryHeap;

use minilisp_data_structures::{
    append, car, cdr, list, AsFloat, AsInteger, AsUnsignedInteger, Cell,
    Float, Integer, UnsignedInteger, Value,
};
use minilisp_util::try_result;

// use crate::helpers::{
//     unpack_float_items, unpack_integer_items, unpack_unsigned_integer_items,
// };
use crate::{
    impl_arithmetic_operation, runtime_error, with_caller, Error, ErrorType,
    Result, VirtualMachine,
};

impl_arithmetic_operation!(+ add);
impl_arithmetic_operation!(-sub);
impl_arithmetic_operation!(*mul);
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
                Value::UnsignedInteger(first)=> {
                    let first = first.clone();
                    let mut operands = cdr(&list).into_iter()
                        .filter(|value|value.is_unsigned_integer())
                        .map(|value|value.as_unsigned_integer());
                    Ok(Value::UnsignedInteger(operands.fold(first, |lhs, rhs| lhs $operator rhs)))
                },
                Value::Integer(first)=> {
                    let first = first.clone();
                    let mut operands = cdr(&list).into_iter()
                        .filter(|value|value.is_integer())
                        .map(|value|value.as_integer());
                    Ok(Value::Integer(operands.fold(first, |lhs, rhs| lhs $operator rhs)))
                },
                Value::Float(first)=> {
                    let first = first.clone();
                    let mut operands = cdr(&list).into_iter()
                        .filter(|value|value.is_float())
                        .map(|value|value.as_float());
                    Ok(Value::Float(operands.fold(first, |lhs, rhs| lhs $operator rhs)))
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
