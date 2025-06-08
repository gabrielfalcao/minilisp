#![allow(unused)]
#![feature(trait_alias)]

use std::borrow::Cow;

pub use errors::{Error, ErrorType, Result};
pub mod builtin;
pub mod errors;
pub mod macros;
use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::collections::{BTreeMap, BinaryHeap, VecDeque};
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
pub mod helpers;
pub use helpers::{
    runtime_error, unpack_float_items, unpack_integer_items, unpack_unsigned_integer_items,
};
use minilisp_parser::{Item, Value};
use minilisp_util::{dbg, extend_lifetime, format_to_str, unexpected, with_caller}; //BinaryHeap;

pub type BuiltinFunction =
    for<'c> fn(&mut VirtualMachine<'c>, VecDeque<Item<'c>>) -> Result<Item<'c>>;

#[derive(Clone)]
pub enum Symbol<'c> {
    Item(Item<'c>),
    BuiltinFunction(Cow<'c, str>, BuiltinFunction),
}

impl<'c> Debug for Symbol<'c> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Symbol::Item(item) => format!("Symbol::Item({:#?})", item),
                Symbol::BuiltinFunction(sym, function) =>
                    format!("Symbol::BuiltinFunction({})", sym),
            }
        )
    }
}
impl<'c> Symbol<'c> {
    pub fn as_item(&self) -> Item<'c> {
        match self {
            Symbol::Item(item) => item.clone(),
            Symbol::BuiltinFunction(sym, function) => {
                warn!(format!("symbol {} is a builtin-function", sym));
                Item::symbol(sym)
            },
        }
    }
}
impl<'c> Hash for Symbol<'c> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_item().hash(state)
    }
}
impl<'c> PartialEq for Symbol<'c> {
    fn eq(&self, other: &Symbol<'c>) -> bool {
        self.as_item().eq(&other.as_item())
    }
}
impl<'c> PartialOrd for Symbol<'c> {
    fn partial_cmp(&self, other: &Symbol<'c>) -> Option<Ordering> {
        self.as_item().partial_cmp(&other.as_item())
    }
}

#[derive(Debug, Clone, Default)]
pub struct VirtualMachine<'c> {
    symbols: BTreeMap<String, Symbol<'c>>,
    stack: VecDeque<VirtualMachine<'c>>,
}

impl<'c> VirtualMachine<'c> {
    pub fn new() -> VirtualMachine<'c> {
        let mut vm = VirtualMachine::<'c>::default();
        vm.register_builtin_function("setq", builtin::state::setq);
        vm.register_builtin_function("list", builtin::list::list);
        vm.register_builtin_function("car", builtin::list::car);
        vm.register_builtin_function("cdr", builtin::list::cdr);
        vm.register_builtin_function("cons", builtin::list::cons);
        vm.register_builtin_function("+", builtin::math::arithmetic::add);
        vm.register_builtin_function("-", builtin::math::arithmetic::sub);
        vm.register_builtin_function("*", builtin::math::arithmetic::mul);
        vm.register_builtin_function("/", builtin::math::arithmetic::div);
        vm.register_builtin_function("print", builtin::string::print);
        vm
    }

    pub fn register_builtin_function(&mut self, sym: &'c str, function: BuiltinFunction) {
        self.symbols.insert(
            sym.to_string(),
            Symbol::<'c>::BuiltinFunction(Cow::from(sym.to_string()), function),
        );
    }

    pub fn symbols(&self) -> BTreeMap<String, Symbol<'c>> {
        self.symbols.clone()
    }

    pub fn get_symbol_function(&mut self, sym: &str) -> Result<BuiltinFunction> {
        let symbol = try_result!(self.get_symbol(sym)).clone();
        match symbol {
            Symbol::Item(item) => Err(with_caller!(runtime_error(
                format!("symbol {:#?} is not a function: {:#?}", sym, item),
                None
            ))),

            Symbol::BuiltinFunction(_sym, function) => Ok(function),
        }
    }

    pub fn get_symbol(&mut self, sym: &str) -> Result<&Symbol<'c>> {
        if !self.symbols.contains_key(sym) {
            self.symbols.insert(sym.to_string(), Symbol::Item(Item::symbol(sym)));
        }
        if let Some(symbol) = self.symbols.get(sym) {
            Ok(symbol)
        } else {
            Err(with_caller!(runtime_error(format!("undefined symbol: {:#?}", sym), None)))
        }
    }

    pub fn eval_symbol_function(
        &mut self,
        sym: &str,
        list: VecDeque<Item<'c>>,
    ) -> Result<Value<'c>> {
        // let mut function: Fn(&'c mut VirtualMachine<'c>, VecDeque<Item<'c>>) -> Result<Item<'c>> =
        let mut function = try_result!(self.get_symbol_function(sym));
        let mut vm = &mut self.clone();
        let result = function(vm, list);
        // let result = function.call(
        //     unsafe { std::mem::transmute::<&mut VirtualMachine<'c>, &'c mut VirtualMachine<'c>>(&mut vm) },
        //     list,
        // );
        // if vm.symbols() != self.symbols() {
        //     warn!("time to re-architect the virtual-machine design");
        // }
        match result {
            Ok(item) => Ok(self.eval_ast(item)?),
            Err(error) => Err(runtime_error(
                format!("Failed to evaluate function {:#?}: {}", sym, error),
                Some(with_caller!(error)),
            )),
        }
    }

    pub fn eval_ast(&mut self, mut item: Item<'c>) -> Result<Value<'c>> {
        match item {
            Item::List(mut list) => {
                if list.is_empty() {
                    return Ok(Value::Nil);
                }
                let car = list.pop_front().unwrap();
                match car {
                    Item::Symbol(symbol) =>
                        Ok(try_result!(self.eval_symbol_function(&symbol, list))),
                    item => unexpected!(item),
                }
            },
            Item::Symbol(symbol) => Ok(try_result!(self.eval_symbol(&symbol))),
            Item::Value(value) => Ok(value),
        }
    }

    pub fn setq(&mut self, sym: String, item: Item<'c>) -> Option<Symbol<'c>> {
        //("setq", &item);
        let previous = self.symbols.insert(sym, Symbol::Item(item));
        //("setq", &self.symbols);
        previous
    }

    pub fn eval_list(&mut self, mut list: VecDeque<Item<'c>>) -> Result<Value<'c>> {
        //(&list);
        if list.is_empty() {
            Ok(Value::Nil)
        } else {
            match list.front().clone() {
                Some(Item::Symbol(sym)) => {
                    //(&sym);
                    Ok(try_result!(
                        self.eval_symbol_function(sym, list.range(1..).map(Clone::clone).collect())
                    ))
                },
                Some(Item::List(list)) => Ok(try_result!(self.eval_list(list.clone()))),
                Some(Item::Value(value)) => Ok(value.clone()),
                None => {
                    //(list.front());
                    Ok(Value::String(Cow::from(format!("{:#?}", list))))
                },
            }
        }
    }

    pub fn eval_symbol(&mut self, sym: &str) -> Result<Value<'c>> {
        let symbol = try_result!(self.get_symbol(sym));
        //(&sym, &symbol);
        match symbol {
            Symbol::Item(item) => match item {
                Item::Value(value) => Ok(value.clone()),
                Item::List(list) => {
                    //(&list);
                    let debug = format!("{:#?}", list);
                    Ok(Value::String(Cow::from(debug.as_str().to_string())))
                },
                Item::Symbol(item_sym) => {
                    //(&item_sym);
                    let debug = format!("{:#?}", item_sym);
                    Ok(Value::String(Cow::from(debug.as_str().to_string())))
                },
                // if sym != *item_sym {
                //     Err(with_caller!(runtime_error(
                //         format!("when evaluating {:#?}: {} != {}", sym, sym, item_sym),
                //         None
                //     )))
                // } else {
                //     let symbol = try_result!(self.get_symbol(sym));
                //     dbg!(&symbol);
                //     match symbol {
                //         Symbol::Item(Item::Value(value)) => Ok(value.clone()),
                //         Symbol::Item(Item::Symbol(sym)) => {
                //             dbg!(&sym);
                //             Ok(try_result!(self.eval_symbol(sym)))
                //         },
                //         Symbol::Item(Item::List(list)) => {
                //             dbg!(&list);
                //             let debug = format!("{:#?}", list);
                //             Ok(Value::String(Cow::from(debug.as_str().to_string())))
                //         },
                //         Symbol::BuiltinFunction(sym, _) =>
                //             Ok(Value::String(Cow::from(format!("builtin function {:#?}", sym)))),
                //     }
                // },
            },
            Symbol::BuiltinFunction(sym, _) =>
                Ok(Value::String(Cow::from(format!("builtin function {:#?}", sym)))),
        }
    }
}
