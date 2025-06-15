use std::collections::{BTreeMap, VecDeque};
use std::fmt::Debug;

use minilisp_data_structures::{car, cdr, AsValue, Cell, Quotable, Symbol, Value};
use minilisp_parser::parse_source;
use minilisp_util::{try_result, unexpected, with_caller};
use unique_pointer::UniquePointer;

use crate::{
    builtin, info, runtime_error, warn, BuiltinFunction, Function, Result, Sym,
    SymbolTable, VirtualMachine,
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
        // info!("VirtualMachine.new", 178);
        let context = Context { symbols, vm };
        // dbg!(&context);
        context
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
        // info!("get_symbol_function", 187);
        let symbol = try_result!(self.get_symbol(sym)).clone();
        let result = match symbol {
            Sym::Value(item) => {
                // dbg!(&item);
                Ok(None)
            },
            Sym::Function(function) => Ok(Some(function)),
        };
        // dbg!(&self.symbols, &sym, &result);
        result
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
        // info!("eval_symbol_function", 196);
        // dbg!(&sym, &list);
        let vm = UniquePointer::read_only(self);

        match try_result!(self.get_symbol_function(sym)) {
            Some(function) => {
                // dbg!(&sym, &list);
                let result = function.call(vm, list);
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
                // dbg!(&sym, &list);
                for item in list.into_iter() {
                    cell.push_value(item);
                }
                cell
            })),
        }
    }

    pub fn eval_string(&mut self, string: &'c str) -> Result<Value<'c>> {
        // info!(format!("Context.eval_string {:#?}", &string), 202);
        Ok(try_result!(self.eval(try_result!(parse_source(string)))))
    }

    pub fn eval(&mut self, item: Value<'c>) -> Result<Value<'c>> {
        // info!(format!("Context.eval {:#?}", &item), 9);
        // dbg!(&self, &item);
        if item.is_quoted() {
            return Ok(item);
        }
        // dbg!(&self, &item);
        match &item {
            Value::List(_) | Value::QuotedList(_) =>
                Ok(try_result!(self.eval_list(item))),
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
        // info!(format!("set_global {} {}", &sym, &item), 125);

        Ok(try_result!(self.symbols.set_global(
            UniquePointer::read_only(self),
            sym,
            item
        )))
    }

    pub fn set_local(&mut self, sym: &Symbol<'c>, item: &Sym<'c>) -> Result<Value<'c>> {
        // info!(format!("set_local {} {}", &sym, &item), 206);
        Ok(try_result!(self.symbols.set_local(
            UniquePointer::read_only(self),
            sym,
            item
        )))
    }

    pub fn eval_list(&mut self, list: Value<'c>) -> Result<Value<'c>> {
        // info!(format!("eval_list {}", &list), 82);
        // dbg!(&list);
        if list.is_quoted() {
            return Ok(list);
        }

        if list.is_empty() {
            Ok(Value::Nil)
        } else {
            match car(&list) {
                Value::Symbol(ref sym) | Value::QuotedSymbol(ref sym) => {
                    // dbg!(&sym);
                    Ok(try_result!(self.eval_symbol_function(sym, cdr(&list))))
                },
                Value::List(_) | Value::QuotedList(_) => {
                    let mut cell = Cell::nil();
                    for item in list.clone().into_iter() {
                        // dbg!(&item);
                        cell.push_value(try_result!(self.eval(item.clone())));
                    }
                    Ok(Value::List(cell))
                },
                _ => Ok(list),
            }
        }
    }

    pub fn eval_symbol(
        &mut self,
        sym: &Symbol<'c>,
        args: Value<'c>,
    ) -> Result<Value<'c>> {
        // info!("eval_symbol");
        // dbg!(&sym, &args);
        let mut vm = UniquePointer::read_only(self);
        let symbol = try_result!(vm.inner_mut().get_symbol(sym));
        let function = sym;
        // dbg!(&function, &args);
        match symbol {
            Sym::Value(value) => Ok(value.clone()),
            Sym::Function(function) => {
                //
                Ok(try_result!(function.call(vm, args)))
            },
        }
    }
}
