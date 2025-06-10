use std::borrow::Cow;
pub mod errors;
pub use errors::{Caller, Error, Result};
pub mod ast;
pub mod macros;
pub mod test;
use std::collections::VecDeque;

pub mod source;
pub use ast::{
    format_position,
    format_rule,
    format_span, //, parse_error_expecting, rule_options_to_string, rule_to_string,
    Item,
    Node,
    Value,
};
use pest::Parser;
use pest_derive::Parser;
pub use source::{Source, Span, SpanPosition};

pub const GRAMMAR: &'static str = include_str!("./grammar.pest");

#[derive(Parser, Debug, Clone)]
#[grammar = "src/grammar.pest"]
pub struct MinilispSource;
pub fn parse_source<'a, T: std::fmt::Display>(input: T) -> Result<'a, VecDeque<Node<'a>>> {
    let input = input.to_string();
    let source_info = Source {
        source: Cow::from(input.clone()),
        filename: None,
    };
    let source = source_info.clone();
    let mut pairs = MinilispSource::parse(Rule::file, unsafe {std::mem::transmute::<&str, &'a str>(input.as_str())}).map_err(|e| {
        Error::new(e.variant.message().to_string(), Some(Span::from_error(e, source_info.clone())))
    })?;
    let file = pairs.next().unwrap();
    let nodes = file
        .into_inner()
        .next()
        .unwrap()
        .into_inner()
        .map(|pair| Node::from_pair(pair.clone(), source.clone()))
        .collect::<VecDeque<Node<'a>>>();
    Ok(nodes)
}
