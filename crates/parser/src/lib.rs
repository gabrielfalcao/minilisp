#![allow(unused, non_snake_case)]
pub mod errors;
pub use errors::{Caller, Error, Result};
pub mod ast;
pub mod test;

pub use ast::{
    format_position,
    format_rule,
    format_span, //, parse_error_expecting, rule_options_to_string, rule_to_string,
    Item,
    Node,
    NodePosition,
    Source,
    Value,
};
use minilisp_formatter::highlight_code_string;
use minilisp_util::{extend_lifetime, string_to_str, try_result};
use pest::error::LineColLocation;
use pest::iterators::{Pair, Pairs};
use pest::{Parser, RuleType};
use pest_derive::Parser;

pub const GRAMMAR: &'static str = include_str!("./grammar.pest");

#[derive(Parser, Debug, Clone)]
#[grammar = "src/grammar.pest"]
pub struct MinilispSource;
pub fn parse_source<'a>(input: &str) -> Result<'a, Vec<Item<'a>>> {
    let source_info = Source {
        source: string_to_str!(&input.to_string(), 'a),
        filename: None,
    };

    let mut pairs = {
        let source = source_info.clone();
        MinilispSource::parse(Rule::file, input).map_err(move |e| {
            Error::new(
                e.variant.message().to_string(),
                Some(Node::from_error(e, extend_lifetime!(&'a Source, &source))),
            )
        })
    }?;
    let mut file = pairs.next().unwrap();
    // eprintln!("{:#?}", Node::from_pair(&file, &source_info));
    let mut statement = file.into_inner().next().unwrap();
    // eprintln!("{:#?}", Node::from_pair(&statement, &source_info));
    // let items = statement
    //     .clone()
    //     .into_inner()
    //     .map(|pair| (pair, extend_lifetime!(&'a Source, &source_info)))
    //     .map(|(pair, source)| Node::from_pair(extend_lifetime!(&'a Pair<Rule>, &pair), source))
    //     .collect::<Vec<Node>>();
    let items = statement
        .into_inner()
        .map(|mut pair| Item::from_pair(&mut pair))
        .collect::<Vec<Item<'a>>>();
    Ok(items)
}
