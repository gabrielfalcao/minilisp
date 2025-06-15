use std::collections::{BTreeMap, VecDeque};
use std::fmt::Debug;

use minilisp_data_structures::{car, cdr, AsValue, Cell, Symbol, Value};
use minilisp_parser::parse_source;
use minilisp_util::{dbg, try_result, unexpected, with_caller};
use unique_pointer::UniquePointer;

use crate::{builtin, runtime_error, BuiltinFunction, Context, Function, Result, Sym};

#[derive(Clone)]
pub struct VirtualMachine<'c> {
    symbols: BTreeMap<Symbol<'c>, Sym<'c>>,
    stack: VecDeque<UniquePointer<Context<'c>>>,
}

impl<'c> Debug for VirtualMachine<'c> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "VirtualMachine {{
symbols: {:#?},
stack_size: {:#?}
}}",
            &self.symbols,
            self.stack.len()
        )
    }
}

impl<'c> VirtualMachine<'c> {
    pub fn new() -> VirtualMachine<'c> {
        VirtualMachine {
            symbols: crate::symbol_table::new(),
            stack: VecDeque::new(),
        }
    }

    fn push_context(&mut self) -> UniquePointer<Context<'c>> {
        let context = UniquePointer::<Context<'c>>::from(Context::new(
            UniquePointer::read_only(self),
            self.symbols.clone(),
        ));
        self.stack.push_front(context.clone());
        context
    }

    pub fn eval_string(&mut self, string: &'c str) -> Result<Value<'c>> {
        Ok(try_result!(self.push_context().eval_string(string)))
    }

    pub fn eval(&mut self, item: Value<'c>) -> Result<Value<'c>> {
        Ok(try_result!(self.push_context().eval(item)))
    }
}
