#![allow(unused, non_snake_case)]
pub mod errors;
pub use errors::{Caller, Error, Result};
pub mod ast;
pub mod test;

pub use ast::{
    format_position,
    format_rule,
    format_span, //, parse_error_expecting, rule_options_to_string, rule_to_string,
    NodeInfo,
    NodePosition,
    SourceInfo,
};
use minilisp_formatter::highlight_code_string;
use minilisp_util::{string_to_str, try_result};
use pest::error::LineColLocation;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser, Debug, Clone)]
#[grammar = "src/grammar.pest"]
pub struct MinilispSource;
pub fn parse_source<'a>(input: &str) -> Result<'a, SourceInfo<'a>> {
    let source_info = SourceInfo {
        source: string_to_str!(input, 'a),
        filename: None,
    };

    let mut source = MinilispSource::parse(Rule::file, input).map_err(|e| {
        let (start_pos, end_pos) = match e.line_col.clone() {
            LineColLocation::Pos(line_col) => (
                NodePosition::from_tuple(line_col.clone()),
                NodePosition::from_tuple(line_col.clone()),
            ),
            LineColLocation::Span(start_pos, end_pos) =>
                (NodePosition::from_tuple(start_pos), NodePosition::from_tuple(end_pos)),
        };
        return Error::new(
            e.variant.message().to_string(),
            Some(NodeInfo {
                input: string_to_str!(input, 'a),
                string: string_to_str!(e.line(), 'a),
                start_pos,
                end_pos,
                source: source_info.clone(),
            }),
        );
    })?;

    eprintln!("{}", highlight_code_string(format!("{:#?}", &source))?);
    Ok(source_info)
}
