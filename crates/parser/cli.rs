#![allow(unused)]
use minilisp_parser::test::stub_input;
use minilisp_parser::{parse_source, Result, GRAMMAR};

pub fn grammar() {
    match pest_meta::parse_and_optimize(GRAMMAR) {
        Ok((strings, rules)) => {
            dbg!(strings);
            dbg!(rules);
        },
        Err(errors) => {
            dbg!(errors);
        }
    }
}

fn main() -> Result<'static, ()>{
    let input = r#"

(cons "a" "b")

"#.trim();

    let _item = parse_source(&input)?;
    Ok(())
}
