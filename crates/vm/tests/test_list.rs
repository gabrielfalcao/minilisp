#![allow(unused)]
use k9::assert_equal;
use minilisp_data_structures::{list, Value};
use minilisp_vm::{Result, VirtualMachine};

#[test]
fn test_car() -> Result<()> {
    let mut vm = VirtualMachine::new();
    let ast = list([
        Value::symbol("car"),
        list([
            Value::symbol("a"),
            Value::symbol("b"),
            Value::symbol("c"),
        ]),
    ]);

    let val = vm.eval(ast)?;
    assert_equal!(val, Value::symbol("a"));
    Ok(())
}
