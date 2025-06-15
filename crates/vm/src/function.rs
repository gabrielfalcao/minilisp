use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};

use minilisp_data_structures::{
    append, AsSymbol, AsValue, Symbol, Value,
};
use minilisp_util::{try_result, with_caller};
use unique_pointer::UniquePointer;

use crate::{runtime_error, Result, Context, BuiltinFunction, warn};

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
    ) -> Result<()> {
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
            Ok(())
        }
    }

    pub fn call(
        &self,
        mut vm: UniquePointer<Context<'c>>,
        list: Value<'c>,
    ) -> Result<Value<'c>> {
        warn!(format!("\ncalling {:#?}", &self));
        match self {
            Function::Defun { name, args, body } => {
                try_result!(self.validate_args(name, args, &list));
                // bind args
                for (symbol, value) in args
                    .clone()
                    .into_iter()
                    .zip(list.clone().into_iter())
                {
                    try_result!(vm.inner_mut().setq(symbol.as_symbol(), value));
                }
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

#[derive(Clone)]
pub enum Sym<'c> {
    Value(Value<'c>),
    Function(Function<'c>),
}

impl<'c> Debug for Sym<'c> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Sym::Value(value) => format!("{:#?}", value),
                Sym::Function(function) => format!("{:#?}", function),
            }
        )
    }
}
impl<'c> Sym<'c> {
    pub fn as_value(&self) -> Value<'c> {
        match self {
            Sym::Value(value) => value.clone(),
            Sym::Function(Function::Builtin { name, function }) => Value::symbol(name),
            Sym::Function(Function::Defun { name, args, body }) => Value::list([
                Value::from(name),
                args.clone(),
                append(body.clone()),
            ]),
        }
    }
}
impl<'c> Hash for Sym<'c> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_value().hash(state)
    }
}
impl<'c> PartialEq for Sym<'c> {
    fn eq(&self, other: &Sym<'c>) -> bool {
        self.as_value().eq(&other.as_value())
    }
}
impl<'c> PartialOrd for Sym<'c> {
    fn partial_cmp(&self, other: &Sym<'c>) -> Option<Ordering> {
        self.as_value().partial_cmp(&other.as_value())
    }
}
