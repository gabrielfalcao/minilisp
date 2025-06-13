#![allow(unused)]
#![feature(trait_alias)]

use std::borrow::Cow;

pub use errors::{Error, ErrorType, Result};
pub mod builtin;
pub mod errors;
pub mod macros;
use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::collections::{BTreeMap, VecDeque};
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};

pub mod helpers;
pub use helpers::{
    runtime_error, unpack_float_items, unpack_integer_items, unpack_unsigned_integer_items,
};
use minilisp_data_structures::{append, car, cdr, list, Cell, Value, Symbol};
use minilisp_util::{format_to_str, unexpected, with_caller};

pub type BuiltinFunction =
    for<'c> fn(&mut VirtualMachine<'c>, Value<'c>) -> Result<Value<'c>>;

#[derive(Clone)]
pub enum Sym<'c> {
    Value(Value<'c>),
    BuiltinFunction(Symbol<'c>, BuiltinFunction),
}

impl<'c> Debug for Sym<'c> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Sym::Value(value) => format!("{:#?}", value),
                Sym::BuiltinFunction(name, _) => format!("builtin-function"),
            }
        )
    }
}
impl<'c> Sym<'c> {
    pub fn as_value(&self) -> Value<'c> {
        match self {
            Sym::Value(value) => value.clone(),
            Sym::BuiltinFunction(sym, function) => {
                warn!(format!("symbol {} is a builtin-function", sym));
                Value::symbol(sym)
            },
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

#[derive(Debug, Clone, Default)]
pub struct VirtualMachine<'c> {
    symbols: BTreeMap<Symbol<'c>, Sym<'c>>,
    stack: VecDeque<VirtualMachine<'c>>,
}

impl<'c> VirtualMachine<'c> {
    pub fn new() -> VirtualMachine<'c> {
        let mut vm = VirtualMachine::<'c>::default();
        // state side-effect functions
        vm.register_builtin_function("setq", builtin::state::setq);

        // list functions
        vm.register_builtin_function("car", builtin::list::car);
        vm.register_builtin_function("cdr", builtin::list::cdr);
        vm.register_builtin_function("cons", builtin::list::cons);
        vm.register_builtin_function("list", builtin::list::list);
        vm.register_builtin_function("quote", builtin::list::quote);
        vm.register_builtin_function("print", builtin::string::print);
        vm.register_builtin_function("backquote", builtin::list::backquote);

        // arithmetic functions
        vm.register_builtin_function("*", builtin::math::arithmetic::mul);
        vm.register_builtin_function("+", builtin::math::arithmetic::add);
        vm.register_builtin_function("-", builtin::math::arithmetic::sub);
        vm.register_builtin_function("/", builtin::math::arithmetic::div);
        vm
    }

    pub fn register_builtin_function(
        &mut self,
        sym: &'c str,
        function: BuiltinFunction,
    ) {
        self.symbols.insert(
            Symbol::new(sym),
            Sym::<'c>::BuiltinFunction(Symbol::new(sym), function),
        );
    }

    pub fn symbols(&self) -> BTreeMap<Symbol<'c>, Sym<'c>> {
        self.symbols.clone()
    }

    pub fn get_symbol_function(
        &mut self,
        sym: &str,
    ) -> Result<BuiltinFunction> {
        let symbol = try_result!(self.get_symbol(sym)).clone();
        match symbol {
            Sym::Value(item) => Err(with_caller!(runtime_error(
                format!("symbol {:#?} is not a function: {:#?}", sym, item),
                None
            ))),
            Sym::Value(value) => Err(with_caller!(runtime_error(
                format!("symbol {:#?} is not a function: {:#?}", sym, value),
                None
            ))),

            Sym::BuiltinFunction(_sym, function) => Ok(function),
        }
    }

    pub fn get_symbol(&mut self, sym: &str) -> Result<&Sym<'c>> {
        let sym = Symbol::new(sym);
        if !self.symbols.contains_key(&sym) {
            self.symbols
                .insert(sym.clone(), Sym::Value(Value::symbol(&sym)));
        }
        if let Some(symbol) = self.symbols.get(&sym) {
            Ok(symbol)
        } else {
            Err(with_caller!(runtime_error(
                format!("undefined symbol: {:#?}", sym),
                None
            )))
        }
    }

    pub fn eval_symbol_function(
        &mut self,
        sym: &Symbol<'c>,
        list: Value<'c>,
    ) -> Result<Value<'c>> {
        let mut function = try_result!(self.get_symbol_function(sym.symbol()));
        let result = function(self, list);
        match result {
            Ok(item) => Ok(self.eval_ast(item)?),
            Err(error) => Err(runtime_error(
                format!("Failed to evaluate function {:#?}: {}", sym, error),
                Some(with_caller!(error)),
            )),
        }
    }

    pub fn eval_ast(&mut self, item: Value<'c>) -> Result<Value<'c>> {
        match &item {
            Value::List(ref list) => {
                if list.is_empty() {
                    return Ok(Value::Nil);
                }
                match car(&item) {
                    Value::Symbol(ref symbol) => Ok(try_result!(
                        self.eval_symbol_function(symbol, cdr(&item))
                    )),
                    _ => Ok(car(&item)),
                }
            },
            Value::Symbol(symbol) =>
                Ok(try_result!(self.eval_symbol(symbol.symbol()))),
            value => Ok(value.clone()),
        }
    }

    pub fn setq(&mut self, sym: Symbol<'c>, item: Value<'c>) -> Result<Value<'c>> {
        //("setq", &item);
        let symbol_value = Sym::Value(self.eval_ast(item.clone())?);
        let previous = self
            .symbols
            .insert(sym.clone(), symbol_value.clone());
        //("setq", &self.symbols);
        let item = match previous.unwrap_or_else(|| symbol_value) {
            Sym::Value(value) => value.clone(),
            Sym::BuiltinFunction(sym, _) => Value::symbol(sym.clone()),
        };

        Ok(item)
    }

    pub fn eval_list(&mut self, mut list: Value<'c>) -> Result<Value<'c>> {
        //(&list);
        if list.is_empty() {
            Ok(Value::Nil)
        } else {
            match car(&list) {
                Value::Symbol(ref sym) => {
                    //(&sym);
                    Ok(try_result!(
                        self.eval_symbol_function(sym, cdr(&list))
                    ))
                },
                Value::List(ref list) =>
                    Ok(try_result!(self.eval_list(Value::list(list.clone())))),
                value => {
                    unexpected!(value)
                }
            }
        }
    }

    pub fn eval_symbol(&mut self, sym: &str) -> Result<Value<'c>> {
        let symbol = try_result!(self.get_symbol(sym));
        //(&sym, &symbol);
        match symbol {
            Sym::Value(value) => Ok(value.clone()),
            Sym::BuiltinFunction(sym, _) =>
                Ok(Value::string(format!("builtin function {:#?}", sym))),
        }
    }
}
