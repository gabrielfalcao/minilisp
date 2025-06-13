use minilisp_data_structures::{Cell, Symbol, Value};
use minilisp_util::{dbg, try_result};

use crate::{
    impl_unpack_values, with_caller, Error, ErrorType, Result, Sym,
    VirtualMachine,
};

impl_unpack_values!(unpack_unsigned_integer_items, is_unsigned_integer);
impl_unpack_values!(unpack_integer_items, is_integer);
impl_unpack_values!(unpack_float_items, is_float);

pub fn runtime_error(message: String, previous: Option<Error>) -> Error {
    with_caller!(Error::with_previous_error(
        message,
        ErrorType::RuntimeError,
        previous
    ))
}

#[macro_export]
macro_rules! impl_unpack_values {
    ($function_name:ident, $filter_function:ident) => {
        pub fn $function_name<'c>(
            vm: &mut VirtualMachine,
            list: Value<'c>,
        ) -> Result<Value<'c>> {
            let vm = unsafe {
                std::mem::transmute::<
                    &mut VirtualMachine,
                    &'c mut VirtualMachine,
                >(vm)
            };
            Ok(list
                .clone()
                .into_iter()
                .filter(|value| value.$filter_function())
                .collect::<Value<'c>>())
        }
    };
}
