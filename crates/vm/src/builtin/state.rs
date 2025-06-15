 //BinaryHeap;

use minilisp_data_structures::{
    car, cdr, AsSymbol, Value,
};
use unique_pointer::UniquePointer;

use crate::helpers::runtime_error;
use crate::{Result, Context, Sym};

pub fn setq<'c>(
    mut vm: UniquePointer<Context<'c>>,
    list: Value<'c>,
) -> Result<Value<'c>> {
    // let list = vm.eval_list_as_items(list)?;
    if list.len() % 2 != 0 {
        return Err(runtime_error(
            format!(
                "odd number of arguments ({}) in setq: {:#?}",
                list.len(),
                list
            ),
            None,
        ));
    }
    let head = car(&list);
    if !head.is_symbol() {
        return Err(runtime_error(
            format!("setq invoked with non-symbol: {:#?}", head),
            None,
        ));
    }
    Ok(vm.set_global(&head.as_symbol(), &Sym::Value(cdr(&list)))?)
}


pub fn defun<'c>(
    mut vm: UniquePointer<Context<'c>>,
    list: Value<'c>,
) -> Result<Value<'c>> {
    let name = car(&list).as_symbol();
    let args = car(&cdr(&list));
    let body = cdr(&cdr(&list));
    Ok(vm.register_function(name, args, body))
}
