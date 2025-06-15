use std::collections::{BTreeMap, VecDeque};
use std::fmt::Debug;

use minilisp_data_structures::{
    car, cdr, AsValue, Cell, Quotable, Symbol, Value,
};
use minilisp_parser::parse_source;
use minilisp_util::{try_result, unexpected, with_caller};
use unique_pointer::UniquePointer;

use crate::{
    builtin, runtime_error, warn, BuiltinFunction, Context, Function, Result, Sym,
    SymbolTable,
};

#[derive(Clone)]
pub struct VirtualMachine<'c> {
    symbols: SymbolTable<'c>,
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
            symbols: SymbolTable::new(),
            stack: VecDeque::new(),
        }
    }

    pub(crate) fn push_context(&mut self) -> UniquePointer<Context<'c>> {
        let context = UniquePointer::<Context<'c>>::from(Context::new(
            UniquePointer::read_only(self),
            self.symbols.clone(),
        ));
        self.stack.push_front(context.clone());
        context
    }

    pub(crate) fn last_context(&self) -> Option<&UniquePointer<Context<'c>>> {
        self.stack.front()
    }

    pub(crate) fn update_symbols(&mut self) {
        if let Some(context) = self.last_context() {
            self.symbols.extend(context.symbols.clone());
        }
    }

    pub fn eval_string(&mut self, string: &'c str) -> Result<Value<'c>> {
        warn!(format!("\neval_string {:#?}", &string));
        let value = try_result!(self.push_context().eval_string(string));
        self.update_symbols();
        Ok(value)
    }

    pub fn eval(&mut self, item: Value<'c>) -> Result<Value<'c>> {
        warn!(format!("eval {:#?}", &item));
        let value = try_result!(self.push_context().eval(item));
        self.update_symbols();
        Ok(value)
    }

    pub fn eval_list(&mut self, list: Value<'c>) -> Result<Value<'c>> {
        let value = try_result!(self.push_context().eval_list(list));
        self.update_symbols();
        Ok(value)
    }

    pub fn eval_symbol_function(
        &mut self,
        sym: &Symbol<'c>,
        list: Value<'c>,
    ) -> Result<Value<'c>> {
        let value = try_result!(self
            .push_context()
            .eval_symbol_function(sym, list));
        self.update_symbols();
        Ok(value)
    }
}
