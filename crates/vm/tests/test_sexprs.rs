#![allow(unused)]
use k9::assert_equal;
#[rustfmt::skip]
use minilisp_data_structures::{
    AsNumber, AsCell, AsValue, Quotable,
};
#[rustfmt::skip]
use minilisp_data_structures::{
    Cell, Value,
};
#[rustfmt::skip]
use minilisp_vm::{assert_eval_display, Result};
#[rustfmt::skip]
use minilisp_data_structures::{append, car, cdr, cons, list, setcar, setcdr, assert_display_equal};

#[test]
fn test_list_quoted_sexprs() -> Result<()> {
    assert_eval_display!(
        "(list 'a 'b 'c)" => "'(a b c)"
    );
    assert_eval_display!(
        "(list '(x y z) 3) " => " '((x y z) 3)"
    );
    assert_eval_display!(
        "(list 'a 'b 'c) " => " (a b c)"
    );
    assert_eval_display!(
        "'(x y z)" => "'((x y z) 3)"
    );
    assert_eval_display!(
        "(list '(x y z) 3) " => " '('(x y z) 3)"
    );
    Ok(())
}
