#![allow(unused, non_snake_case)]
pub mod errors;
pub use errors::{Caller, Error, Result};
pub mod ast;
pub mod test;

pub use ast::{
    format_position,
    format_rule,
    format_span, //, parse_error_expecting, rule_options_to_string, rule_to_string,
    Node,
    NodePosition,
    Source,
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
pub fn parse_source<'a>(input: &str) -> Result<'a, Vec<Node<'a>>> {
    let source_info = Source {
        source: string_to_str!(input, 'a),
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
    let file = pairs.clone().next().unwrap();
    // eprintln!("{:#?}", Node::from_pair(&file, &source_info));
    let statement = file.clone().into_inner().next().unwrap();
    // eprintln!("{:#?}", Node::from_pair(&statement, &source_info));
    let items = statement
        .clone()
        .into_inner()
        .map(|pair| (pair, extend_lifetime!(&'a Source, &source_info)))
        .map(|(pair, source)| Node::from_pair(extend_lifetime!(&'a Pair<Rule>, &pair), source))
        .collect::<Vec<Node>>();
    // eprintln!("items: {:#?}", &items);
    // let items = statement.into_inner().next().unwrap().into_inner()

    // let items = pairs.next().unwrap().into_inner().next().unwrap().into_inner()
    //     .map(|pair| Node::from_pair(extend_lifetime!(&'a Pair<Rule>, &pair.clone()), &source_info))
    //     .collect::<Vec<Node>>();
    // let nodes = pairs.next().unwrap().into_inner().next().unwrap().into_inner()
    //     .map(|pair| Node::from_pair(pair, &source_info))
    //     .collect::<Vec<Node>>();
    // eprintln!("{:#?}", nodes);
    // eprintln!("{}", highlight_code_string(format!("{:#?}", &nodes))?);
    // grammar();
    Ok(items)
}
