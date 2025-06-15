#![allow(
    unused,
    mutable_transmutes
)]
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
pub mod test;
pub use helpers::runtime_error;
use minilisp_data_structures::{
    append, car, cdr, list, AsSymbol, AsValue, Cell, Symbol, Value,
};
use minilisp_parser::parse_source;
use minilisp_util::{dbg, format_to_str, unexpected, with_caller};
use unique_pointer::UniquePointer;

pub type BuiltinFunction =
    for<'c> fn(UniquePointer<VirtualMachine<'c>>, Value<'c>) -> Result<Value<'c>>;

#[derive(Clone)]
pub enum Sym<'c> {
    Value(Value<'c>),
    BuiltinFunction(Symbol<'c>, BuiltinFunction),
    Function(Symbol<'c>, Value<'c>, Value<'c>),
}

impl<'c> Debug for Sym<'c> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Sym::Value(value) => format!("{:#?}", value),
                Sym::BuiltinFunction(name, _) => format!("builtin-function"),
                Sym::Function(name, args, body) =>
                    format!("(defun {} {} {})", name, args, body),
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
            Sym::Function(name, args, body) => {
                warn!(format!("{} {} {}", &name, &args, &body));
                Value::quoted_list([
                    Value::from(name),
                    args.clone(),
                    body.clone(),
                ])
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
        // identity functions
        vm.register_builtin_function("t", builtin::identity::t);

        // state side-effect functions
        vm.register_builtin_function("setq", builtin::state::setq);
        vm.register_builtin_function("defun", builtin::state::defun);

        // list functions
        vm.register_builtin_function("car", builtin::list::car);
        vm.register_builtin_function("cdr", builtin::list::cdr);
        vm.register_builtin_function("cons", builtin::list::cons);
        vm.register_builtin_function("list", builtin::list::list);
        vm.register_builtin_function("append", builtin::list::append);
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

    pub fn register_function(
        &mut self,
        name: Symbol<'c>,
        args: Value<'c>,
        body: Value<'c>,
    ) -> Value<'c> {
        let function = Sym::<'c>::Function(name.clone(), args, body);
        let previous = self
            .symbols
            .insert(name.clone(), function.clone());
        if let Some(previous) = previous {
            Value::string(format!("previous: {:#?}", previous))
        } else {
            Value::string(format!("{:#?}", function))
        }
    }

    pub fn symbols(&self) -> BTreeMap<Symbol<'c>, Sym<'c>> {
        self.symbols.clone()
    }

    pub fn get_symbol_function(
        &mut self,
        sym: &str,
    ) -> Result<Option<BuiltinFunction>> {
        let symbol = try_result!(self.get_symbol(sym)).clone();
        match symbol {
            Sym::Value(item) => Ok(None),
            Sym::BuiltinFunction(_sym, function) => Ok(Some(function)),
            Sym::Function(name, args, body) => {
                dbg!(&name, &args, &body);
                Err(with_caller!(runtime_error(
                    format!(
                        "symbol {:#?} is not builtin function: (defun {} {} {})",
                        &name, &name, &args, &body
                    ),
                    None
                )))
            },
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
        match try_result!(self.get_symbol_function(sym.symbol())) {
            Some(function) => {
                let result = function(UniquePointer::read_only(self), list);
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
                cell.add(&Cell::from(Value::from(sym)));
                for item in list.into_iter() {
                    cell.add(&Cell::from(item));
                }
                cell
            })),
        }
    }

    pub fn eval_string(&mut self, string: &'c str) -> Result<Value<'c>> {
        Ok(try_result!(self.eval(try_result!(parse_source(string)))))
    }

    pub fn eval(&mut self, item: Value<'c>) -> Result<Value<'c>> {
        match &item {
            Value::List(_) | Value::QuotedList(_) => match car(&item) {
                Value::Symbol(ref symbol) =>
                    Ok(try_result!(self.eval_symbol_function(symbol, cdr(&item)))),
                _ => Ok(item.quote()),
            },
            Value::Symbol(symbol) | Value::QuotedSymbol(symbol) =>
                Ok(try_result!(self.eval_symbol(symbol.symbol()))),
            value => Ok(value.clone()),
        }
    }

    pub fn setq(&mut self, sym: Symbol<'c>, item: Value<'c>) -> Result<Value<'c>> {
        //("setq", &item);
        let symbol_value = Sym::Value(self.eval(item.clone())?);
        let previous = self
            .symbols
            .insert(sym.clone(), symbol_value.clone());
        //("setq", &self.symbols);
        let item = match previous.unwrap_or_else(|| symbol_value) {
            Sym::Value(value) => value.clone(),
            Sym::BuiltinFunction(sym, _) => Value::symbol(sym.clone()),
            Sym::Function(name, args, body) => {
                dbg!(&sym, &item, &name, &body, &args);
                item
            },
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
                    Ok(try_result!(self.eval_symbol_function(sym, cdr(&list))))
                },
                Value::List(_) => {
                    Ok(list.clone())
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

    pub fn eval_defun(
        &mut self,
        sym: Symbol<'c>,
        args: Value<'c>,
        body: Value<'c>,
    ) -> Result<Value<'c>> {
        dbg!(&sym, &args, &body);
        Ok(Value::quoted_list([
            Value::from(sym),
            Value::quoted_list([Value::symbol("args"), args]),
            Value::quoted_list([Value::symbol("body"), body]),
        ]))
    }

    pub fn eval_symbol(&mut self, sym: &str) -> Result<Value<'c>> {
        let symbol =
            try_result!(unsafe {
                std::mem::transmute::<
                    &&mut VirtualMachine<'c>,
                    &mut &mut VirtualMachine<'c>,
                >(&self)
            }
            .get_symbol(sym));
        //(&sym, &symbol);
        match symbol {
            Sym::Value(value) => Ok(value.clone()),
            Sym::BuiltinFunction(sym, _) =>
                Ok(Value::string(format!("builtin function {:#?}", sym))),
            Sym::Function(name, args, body) => Ok(try_result!(unsafe {
                std::mem::transmute::<
                    &&mut VirtualMachine<'c>,
                    &mut &mut VirtualMachine<'c>,
                >(&self)
            }
            .eval_defun(name.clone(), args.clone(), body.clone()))),
        }
    }
}
