use std::collections::{BTreeMap, VecDeque};
use std::fmt::Debug;

use minilisp_data_structures::{car, cdr, AsValue, Cell, Quotable, Symbol, Value};
use minilisp_parser::parse_source;
use minilisp_util::{try_result, unexpected, with_caller};
use unique_pointer::UniquePointer;

use crate::{
    builtin, runtime_error, warn, BuiltinFunction, Function, Result, Sym, SymbolTable,
    VirtualMachine,
};

#[derive(Clone)]
pub struct Context<'c> {
    pub(crate) symbols: SymbolTable<'c>,
    pub(crate) vm: UniquePointer<VirtualMachine<'c>>,
}

impl<'c> Debug for Context<'c> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Context {{
    symbols: {:#?},
}}",
            &self.symbols,
        )
    }
}

impl<'c> Context<'c> {
    pub fn new(
        vm: UniquePointer<VirtualMachine<'c>>,
        symbols: SymbolTable<'c>,
    ) -> Context<'c> {
        Context { symbols, vm }
    }

    pub fn register_function(
        &mut self,
        name: Symbol<'c>,
        args: Value<'c>,
        body: Value<'c>,
    ) -> Value<'c> {
        let function = Sym::<'c>::Function(Function::Defun {
            name: name.clone(),
            args,
            body,
        });
        self.symbols.set_global(
            UniquePointer::read_only(self),
            &name,
            &function.clone(),
        );
        function.as_value()
    }

    pub fn get_symbol_function(
        &mut self,
        sym: &Symbol<'c>,
    ) -> Result<Option<Function<'c>>> {
        let symbol = try_result!(self.get_symbol(sym)).clone();
        match symbol {
            Sym::Value(item) => Ok(None),
            Sym::Function(function) => Ok(Some(function)),
        }
    }

    pub fn get_symbol(&mut self, sym: &Symbol<'c>) -> Result<Sym<'c>> {
        Ok(try_result!(self
            .symbols
            .get(UniquePointer::read_only(self), sym)))
    }

    pub fn eval_symbol_function(
        &mut self,
        sym: &Symbol<'c>,
        list: Value<'c>,
    ) -> Result<Value<'c>> {
        let vm = UniquePointer::read_only(self);

        match try_result!(self.get_symbol_function(sym)) {
            Some(function) => {
                dbg!(&sym, &list);
                let result = function.call(vm, cdr(&list));
                match result {
                    Ok(item) => Ok(self.eval(item)?),
                    Err(error) => Err(runtime_error(
                        format!("Failed to evaluate function {:#?}: {}", sym, error),
                        Some(with_caller!(error)),
                    )),
                }
            },
            None => Ok(Value::from({
                let mut cell = Cell::nil();
                cell.push_value(Value::from(sym));
                dbg!(&sym, &list);
                for item in list.into_iter() {
                    cell.push_value(item);
                }
                cell
            })),
        }
    }

    pub fn eval_string(&mut self, string: &'c str) -> Result<Value<'c>> {
        warn!("\neval_string");
        Ok(try_result!(self.eval(try_result!(parse_source(string)))))
    }

    pub fn eval(&mut self, item: Value<'c>) -> Result<Value<'c>> {
        if item.is_quoted() {
            return Ok(item);
        }
        warn!("\neval");
        dbg!(&self, &item);
        match &item {
            Value::List(_) | Value::QuotedList(_) =>
                Ok(try_result!(self.eval_list(item))),

            // Value::List(_) | Value::QuotedList(_) => match car(&item) {
            //     Value::Symbol(ref symbol) | Value::QuotedSymbol(ref symbol) =>
            //         Ok(try_result!(self.eval_symbol(symbol, cdr(&item)))),
            //     _ => Ok(item.quote()),
            // },
            Value::Symbol(symbol) | Value::QuotedSymbol(symbol) =>
                Ok(try_result!(self.eval_symbol(symbol, cdr(&item)))),
            value => Ok(value.clone()),
        }
    }

    pub fn set_global(
        &mut self,
        sym: &Symbol<'c>,
        item: &Sym<'c>,
    ) -> Result<Value<'c>> {
        Ok(try_result!(self.symbols.set_global(
            UniquePointer::read_only(self),
            sym,
            item
        )))
    }

    pub fn set_local(&mut self, sym: &Symbol<'c>, item: &Sym<'c>) -> Result<Value<'c>> {
        Ok(try_result!(self.symbols.set_local(
            UniquePointer::read_only(self),
            sym,
            item
        )))
    }

    pub fn eval_list(&mut self, list: Value<'c>) -> Result<Value<'c>> {
        if list.is_quoted() {
            return Ok(list);
        }

        warn!("\neval_list");
        dbg!(&self, &list);
        if list.is_empty() {
            Ok(Value::Nil)
        } else {
            match car(&list) {
                Value::Symbol(ref sym) => {
                    dbg!(&sym);
                    Ok(try_result!(self.eval_symbol_function(sym, cdr(&list))))
                },
                Value::List(_) => {
                    let mut cell = Cell::nil();
                    for item in list.clone().into_iter() {
                        dbg!(&item);
                        cell.push_value(try_result!(self.eval(item.clone())));
                    }
                    Ok(Value::List(cell))
                    // let mut value = Value::empty_list();
                    // for result in list.into_iter().map(|value| self.eval(value)) {
                    //     value.extend([try_result!(result)]);
                    // }
                    // Ok(value)
                },
                value => {
                    unexpected!(value)
                },
            }
        }
    }

    pub fn eval_symbol(
        &mut self,
        sym: &Symbol<'c>,
        args: Value<'c>,
    ) -> Result<Value<'c>> {
        let mut vm = UniquePointer::read_only(self);
        let symbol = try_result!(vm.inner_mut().get_symbol(sym));
        let function = sym;
        dbg!(&function, &args);
        match symbol {
            Sym::Value(value) => Ok(value.clone()),
            Sym::Function(function) => {
                //
                Ok(try_result!(function.call(vm, args)))
            },
        }
    }
}
