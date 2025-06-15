use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::fmt::{Debug, Formatter, Display};
use std::hash::{Hash, Hasher};
use std::iter::Zip;

use minilisp_data_structures::{
    append, AsSymbol, AsValue, Symbol, Value, ValueIterator,
};
use minilisp_util::{try_result, with_caller};
use unique_pointer::UniquePointer;

use crate::{runtime_error, admonition, warn, BuiltinFunction, Context, Result, Sym};

#[derive(Clone)]
pub enum Function<'c> {
    Builtin {
        name: Symbol<'c>,
        function: BuiltinFunction,
    },
    Defun {
        name: Symbol<'c>,
        args: Value<'c>,
        body: Value<'c>,
    },
}
impl<'c> Function<'c> {
    pub fn validate_args(
        &self,
        name: &Symbol<'c>,
        expected: &Value<'c>,
        received: &Value<'c>,
    ) -> Result<Zip<ValueIterator<'c>, ValueIterator<'c>>> {
        let expected_length = expected.len();
        let received_length = received.len();
        if expected_length != received_length {
            Err(with_caller!(runtime_error(
                format!(
                    "{} expected {} args but received {}",
                    name, expected_length, received_length
                ),
                None
            )))
        } else {
            Ok(expected
                .clone()
                .into_iter()
                .zip(received.clone().into_iter()))
        }
    }

    pub fn bind_args_to_local_context(
        &self,
        mut vm: UniquePointer<Context<'c>>,
        name: &Symbol<'c>,
        expected: &Value<'c>,
        received: &Value<'c>,
    ) -> Result<()> {
        for (symbol, value) in try_result!(self.validate_args(name, expected, received))
        {
            try_result!(vm
                .inner_mut()
                .set_global(&symbol.as_symbol(), &Sym::Value(value.clone())));
        }
        Ok(())
    }

    pub fn call(
        &self,
        mut vm: UniquePointer<Context<'c>>,
        list: Value<'c>,
    ) -> Result<Value<'c>> {
        admonition!("calling", format!("{:#?}", &self), 58);
        match self {
            Function::Defun { name, args, body } => {
                try_result!(self.bind_args_to_local_context(vm.clone(), name, args, &list));
                let mut value = Value::nil();
                for val in body.clone().into_iter() {
                    dbg!(&value, &name, args, body);
                    value = try_result!(vm.inner_mut().eval(val));
                    dbg!(&value, &name, args, body);
                }
                Ok(value)
            },
            Function::Builtin { name, function } => {
                //
                Ok(try_result!(function(vm, list)))
            },
        }
    }
}

impl<'c> Display for Function<'c> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Function::Defun { name, args, body } =>
                    format!("(defun {} {} {})", name, args, body),
                Function::Builtin { name, function } =>
                    format!("builtin-function {} {:#?}", name, function),
            }
        )
    }
}

impl<'c> Debug for Function<'c> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Function::Defun { name, args, body } =>
                    format!("(defun {} {} {})", name, args, body),
                Function::Builtin { name, function } =>
                    format!("builtin-function {} {:#?}", name, function),
            }
        )
    }
}
