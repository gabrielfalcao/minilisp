#![allow(unused)]
use minilisp_parser::test::stub_input;
use minilisp_parser::{parse_source, Result};
use k9::assert_equal;


#[test]
fn test_parse_cons_of_literals() -> Result<'static, ()> {
    let (input, _) = stub_input(r#"
(cons "a" "b")
"#);
    let item = parse_source(&input)?;
    Ok(())
}
