use std::collections::BTreeMap;

use minilisp_data_structures::Symbol;

use crate::{builtin, BuiltinFunction, Function, Sym};

pub(self) fn register_builtin_function<'c>(
    table: &mut BTreeMap<Symbol<'c>, Sym<'c>>,
    sym: &'c str,
    function: BuiltinFunction,
) {
    let function = Sym::<'c>::Function(Function::Builtin {
        name: Symbol::new(sym),
        function: function,
    });
    table.insert(Symbol::new(sym), function.clone());
}

pub fn debug<'c>(table: &BTreeMap<Symbol<'c>, Sym<'c>>) -> String {
    let mut symbols = BTreeMap::<Symbol<'c>, Sym<'c>>::new();
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
pub fn new<'c>() -> BTreeMap<Symbol<'c>, Sym<'c>> {
    let mut table = BTreeMap::<Symbol<'c>, Sym<'c>>::new();
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
