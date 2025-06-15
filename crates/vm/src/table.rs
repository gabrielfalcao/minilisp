use std::collections::BTreeMap;
use std::fmt::Debug;

use minilisp_data_structures::{AsValue, Cell, Quotable, Symbol, Value};
use minilisp_util::{try_result, unexpected, with_caller};
use unique_pointer::UniquePointer;

use crate::{builtin, BuiltinFunction, Context, Function, Result, Sym, VirtualMachine};

pub type SymTable<'c> = BTreeMap<Symbol<'c>, Sym<'c>>;

#[derive(Clone)]
pub struct SymbolTable<'c> {
    globals: SymTable<'c>,
    locals: SymTable<'c>,
}
impl<'c> Debug for SymbolTable<'c> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "SymbolTable {{
        globals: {},
        locals: {}
    }}",
            &debug(&self.globals),
            &debug(&self.locals),
        )
    }
}

impl<'c> SymbolTable<'c> {
    pub fn new() -> SymbolTable<'c> {
        SymbolTable::with_locals(SymTable::new())
    }

    pub fn with_locals(locals: SymTable<'c>) -> SymbolTable<'c> {
        SymbolTable {
            globals: globals(),
            locals,
        }
    }

    pub fn extend(&mut self, other: Self) {
        self.globals.extend(other.globals.clone())
    }

    pub fn set_global(
        &mut self,
        context: UniquePointer<Context<'c>>,
        sym: &Symbol<'c>,
        item: &Sym<'c>,
    ) -> Result<Value<'c>> {
        Ok(try_result!(set_within_map(&mut self.globals, context, sym, item)))
    }

    pub fn set_local(
        &mut self,
        context: UniquePointer<Context<'c>>,
        sym: &Symbol<'c>,
        item: &Sym<'c>,
    ) -> Result<Value<'c>> {
        Ok(try_result!(set_within_map(&mut self.locals, context, sym, item)))
    }

    pub fn get(
        &mut self,
        context: UniquePointer<Context<'c>>,
        sym: &Symbol<'c>,
    ) -> Result<Sym<'c>> {
        Ok(
            if let Some(value) = self
                .locals
                .get(sym)
                .or_else(|| self.globals.get(sym))
            {
                value.clone()
            } else {
                // trying to get a non-existing symbol places it into
                // the local context
                self.locals
                    .insert(sym.clone(), Sym::Value(sym.as_value()));
                Sym::Value(sym.as_value())
            },
        )
    }
}

fn register_builtin_function<'c>(
    table: &mut SymTable<'c>,
    sym: &'c str,
    function: BuiltinFunction,
) {
    let function = Sym::<'c>::Function(Function::Builtin {
        name: Symbol::new(sym),
        function: function,
    });
    table.insert(Symbol::new(sym), function.clone());
}

pub fn debug<'c>(table: &SymTable<'c>) -> String {
    let mut symbols = SymTable::new();
    for (key, value) in table.into_iter() {
        if let Sym::Value(_) = value.clone() {
            symbols.insert(key.clone(), value.clone());
        }
    }
    format!("{:#?}", symbols)
        .lines()
        .enumerate()
        .map(|(index, line)| {
            format!(
                "{}{}",
                " ".repeat(if index > 0 {
                    4
                } else {
                    0
                }),
                line
            )
        })
        .collect::<Vec<String>>()
        .join("\n")
}
fn globals<'c>() -> SymTable<'c> {
    let mut table = SymTable::new();
    // identity functions
    register_builtin_function(&mut table, "t", builtin::identity::t);

    // state side-effect functions
    register_builtin_function(&mut table, "setq", builtin::state::setq);
    register_builtin_function(&mut table, "defun", builtin::state::defun);

    // list functions
    register_builtin_function(&mut table, "car", builtin::list::car);
    register_builtin_function(&mut table, "cdr", builtin::list::cdr);
    register_builtin_function(&mut table, "cons", builtin::list::cons);
    register_builtin_function(&mut table, "list", builtin::list::list);
    register_builtin_function(&mut table, "append", builtin::list::append);
    register_builtin_function(&mut table, "quote", builtin::list::quote);
    register_builtin_function(&mut table, "print", builtin::string::print);
    register_builtin_function(&mut table, "backquote", builtin::list::backquote);

    // arithmetic functions
    register_builtin_function(&mut table, "*", builtin::math::arithmetic::mul);
    register_builtin_function(&mut table, "+", builtin::math::arithmetic::add);
    register_builtin_function(&mut table, "-", builtin::math::arithmetic::sub);
    register_builtin_function(&mut table, "/", builtin::math::arithmetic::div);
    table
}

fn set_within_map<'c>(
    map: &mut SymTable<'c>,
    context: UniquePointer<Context<'c>>,
    sym: &Symbol<'c>,
    item: &Sym<'c>,
) -> Result<Value<'c>> {
    let previous = map.insert(sym.clone(), item.clone());

    Ok(match previous.unwrap_or_else(|| item.clone()) {
        Sym::Value(value) => {
            dbg!(&(value,), &item, &sym, &context);
            item.clone()
        },
        Sym::Function(Function::Defun { name, args, body }) => {
            dbg!(&(name, args, body), &item, &sym, &context);
            item.clone()
        },
        Sym::Function(Function::Builtin { name, function }) => {
            dbg!(&(name, function), &item, &sym, &context);
            item.clone()
        },
    }
    .as_value())
}
