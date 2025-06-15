use std::collections::BTreeMap;
use std::fmt::Debug;

use minilisp_data_structures::{AsValue, Cell, Quotable, Symbol, Value};
use minilisp_util::{try_result, unexpected, with_caller};
use unique_pointer::UniquePointer;

use crate::{
    builtin, info, warn, BuiltinFunction, Context, Function, Result, Sym,
    VirtualMachine,
};

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
        globals: {:#?},
        locals: {:#?}
    }}",
            &self.globals,
            &self.locals,
            // &debug(&self.globals),
            // &debug(&self.locals),
        )
    }
}

impl<'c> SymbolTable<'c> {
    pub fn new() -> SymbolTable<'c> {
        // info!("SymbolTable.new");
        SymbolTable::with_locals(SymTable::new())
    }

    pub fn with_locals(locals: SymTable<'c>) -> SymbolTable<'c> {
        // info!("SymbolTable.with_locals");
        let mut globals = SymTable::<'c>::new();
        // identity functions
        register_builtin_function(&mut globals, "t", builtin::identity::t);

        // state side-effect functions
        register_builtin_function(&mut globals, "setq", builtin::state::setq);
        register_builtin_function(&mut globals, "defun", builtin::state::defun);

        // list functions
        register_builtin_function(&mut globals, "car", builtin::list::car);
        register_builtin_function(&mut globals, "cdr", builtin::list::cdr);
        register_builtin_function(&mut globals, "cons", builtin::list::cons);
        register_builtin_function(&mut globals, "list", builtin::list::list);
        register_builtin_function(&mut globals, "append", builtin::list::append);
        register_builtin_function(&mut globals, "quote", builtin::list::quote);
        register_builtin_function(&mut globals, "print", builtin::string::print);
        register_builtin_function(&mut globals, "backquote", builtin::list::backquote);

        // arithmetic functions
        register_builtin_function(&mut globals, "*", builtin::math::arithmetic::mul);
        register_builtin_function(&mut globals, "+", builtin::math::arithmetic::add);
        register_builtin_function(&mut globals, "-", builtin::math::arithmetic::sub);
        register_builtin_function(&mut globals, "/", builtin::math::arithmetic::div);

        let mut table = SymbolTable {
            globals: globals.clone(),
            locals,
        };
        // dbg!(&globals, &table);
        table
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
        // info!(format!("SymbolTable.set_global {} {}", &sym, &item), 231);
        Ok(try_result!(set_within_map(&mut self.globals, context, sym, item)))
    }

    pub fn set_local(
        &mut self,
        context: UniquePointer<Context<'c>>,
        sym: &Symbol<'c>,
        item: &Sym<'c>,
    ) -> Result<Value<'c>> {
        // info!(format!("SymbolTable.set_global {} {}", &sym, &item), 16);
        Ok(try_result!(set_within_map(&mut self.locals, context, sym, item)))
    }

    pub fn get(
        &mut self,
        mut vm: UniquePointer<Context<'c>>,
        sym: &Symbol<'c>,
    ) -> Result<Sym<'c>> {
        // info!(format!("SymbolTable.get {:#?}", &sym), 51);
        // dbg!(&sym, &self.globals, &self.locals);
        if let Some(value) = self
            .locals
            .get(sym)
            .map(Clone::clone)
            .or_else(|| self.globals.get(sym).map(Clone::clone))
        {
            // warn!(format!("FOUND {:#?}", &value), 202);
            return Ok(value);
        } else {
            // warn!(format!("NOT FOUND {:#?}", &sym), 33);
            // trying to get a non-existing symbol places it into
            // the local context
            self.locals
                .insert(sym.clone(), Sym::Value(sym.as_value()));
            return Ok(Sym::Value(sym.as_value()));
        }
    }
}

fn register_builtin_function<'c>(
    table: &mut SymTable<'c>,
    sym: &str,
    function: BuiltinFunction,
) {
    let function = Sym::<'c>::Function(Function::Builtin {
        name: Symbol::new(sym),
        function: function,
    });
    table.insert(Symbol::new(sym), function.clone());
}

fn set_within_map<'c>(
    map: &mut SymTable<'c>,
    context: UniquePointer<Context<'c>>,
    sym: &Symbol<'c>,
    item: &Sym<'c>,
) -> Result<Value<'c>> {
    info!(format!("set_within_map {} {}", &sym, &item), 29);
    let previous = map.insert(sym.clone(), item.clone());

    Ok(match previous.unwrap_or_else(|| item.clone()) {
        Sym::Value(value) => {
            dbg!(&(value,), &item, &sym);
            item.clone()
        },
        Sym::Function(Function::Defun { name, args, body }) => {
            dbg!(&(name, args, body), &item, &sym);
            item.clone()
        },
        Sym::Function(Function::Builtin { name, function }) => {
            dbg!(&(name, function), &item, &sym);
            item.clone()
        },
    }
    .as_value())
}
