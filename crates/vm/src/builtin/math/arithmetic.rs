//BinaryHeap;

use minilisp_data_structures::{
    append, car, cdr, AsFloat, AsInteger, AsUnsignedInteger, Cell, Value,
};
use minilisp_util::{dbg, try_result, with_caller};
use unique_pointer::UniquePointer;

// use crate::helpers::{
//     unpack_float_items, unpack_integer_items, unpack_unsigned_integer_items,
// };
use crate::{
    impl_arithmetic_operation, runtime_error, unfold_numeric_values_from_cdr, Result,
    Context,
};

impl_arithmetic_operation!(+ add);
impl_arithmetic_operation!(-sub);
impl_arithmetic_operation!(*mul);
impl_arithmetic_operation!(/ div);

#[macro_export]
macro_rules! unfold_numeric_values_from_cdr {
    (
        $operator:expr,
        $lifetime:lifetime,
        $vm:expr,
        $list:expr,Value::
        $numeric_variant:ident,
        $as_value_fragment_name:ident,
        $is_value_fragment_name:ident
        $(,)?
    ) => {{
        let mut operands = Cell::nil();
        for value in cdr(&$list).into_iter() {
            if value.$is_value_fragment_name() {
                operands.add(&Cell::from(value));
            } else if value.is_list() {
                operands.add(&Cell::from(try_result!($vm.inner_mut().eval(value))));
            } else if value.is_symbol() {
                operands.add(&Cell::from(try_result!($vm.inner_mut().eval(value))));
            } else {
                return Err(with_caller!(runtime_error(
                    format!(
                        "called with unexpected, non-numerical value: {:#?}",
                        value
                    ),
                    None
                )));
            }
        }
        operands.into_iter().map(|value|value.$as_value_fragment_name()) //.collect::<Value<$lifetime>>()
    }};
}

#[macro_export]
macro_rules! impl_arithmetic_operation {
    (
        $operator:tt
            $function_name:ident
    ) => {
        pub fn $function_name<'c>(
            mut vm: UniquePointer<Context<'c>>,
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
                    let operands = unfold_numeric_values_from_cdr!("$operator", 'c, vm, list, Value::UnsignedInteger, as_unsigned_integer, is_unsigned_integer);
                    Ok(Value::UnsignedInteger(operands.fold(first, |lhs, rhs| lhs $operator rhs)))
                },
                Value::Integer(first)=> {
                    let first = first.clone();
                    let operands = unfold_numeric_values_from_cdr!("$operator", 'c, vm, list, Value::Integer, as_integer, is_integer);
                    Ok(Value::Integer(operands.fold(first, |lhs, rhs| lhs $operator rhs)))
                },
                Value::Float(first)=> {
                    let first = first.clone();
                    let operands = unfold_numeric_values_from_cdr!("$operator", 'c, vm, list, Value::Float, as_float, is_float);
                    Ok(Value::Float(operands.fold(first, |lhs, rhs| lhs $operator rhs)))
                },
                Value::Symbol(sym) | Value::QuotedSymbol(sym) => {
                    let first = try_result!(vm.inner_mut().eval(Value::from(sym)));
                    Ok(try_result!($function_name(vm, append([first, cdr(&list)]))))
                },
                Value::List(tmp) | Value::QuotedList(tmp) => {
                    dbg!(&vm.inner_ref(), &tmp);
                    Ok(try_result!($function_name(vm.clone(), append([try_result!(vm.inner_mut().eval(car(&list))), cdr(&list)]))))
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
