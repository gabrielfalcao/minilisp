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
use pest::error::LineColLocation;
use pest::Parser;
use pest_derive::Parser;
use minilisp_util::string_to_str;

#[derive(Parser, Debug, Clone)]
#[grammar = "src/grammar.pest"]
pub struct SimplelangSource;
pub fn parse_source<'a>(input: &str) -> Result<'a, SourceInfo<'a>> {
    Ok(SourceInfo {
        source: string_to_str!(input, 'a),
        filename: None
    })
}
// impl SimplelangSource {
//     pub fn parse_nodes_with_source_info(
//         string: &str,
//         source_info: &SourceInfo,
//     ) -> std::result::Result<Vec<Node>, Error> {
//         let mut source = SimplelangSource::parse(Rule::file, string).map_err(|e| {
//             let fancy_e = e.renamed_rules(|rule| rule_to_string(rule));
//             let (start_pos, end_pos) = match fancy_e.line_col.clone() {
//                 LineColLocation::Pos(line_col) => (
//                     NodePosition::from_tuple(line_col.clone()),
//                     NodePosition::from_tuple(line_col.clone()),
//                 ),
//                 LineColLocation::Span(start_pos, end_pos) =>
//                     (NodePosition::from_tuple(start_pos), NodePosition::from_tuple(end_pos)),
//             };
//             return Error::new(
//                 fancy_e.variant.message().to_string(),
//                 NodeInfo {
//                     input: string.to_string(),
//                     string: fancy_e.line().to_string(),
//                     start_pos,
//                     end_pos,
//                     source: source_info.clone(),
//                 },
//             );
//         })?;
//         let mut nodes = Vec::new();
//         for stmt in source.next().unwrap().into_inner() {
//             Node::expect_rules(vec![Rule::statement, Rule::EOI], &stmt, source_info)?;
//             for expr in stmt.clone().into_inner() {
//                 Node::expect_rules(
//                     vec![Rule::function_definition, Rule::expression],
//                     &expr,
//                     source_info,
//                 )?;
//
//                 match expr.as_rule() {
//                     Rule::function_definition => {
//                         let expression =
//                             Node::from_pair_rule_function_definition(&expr, source_info)?;
//                         nodes.push(Node::FunctionDeclaration(expression));
//                     },
//                     Rule::expression => {
//                         let expression = Node::from_pair_rule_expression(&expr, source_info)?;
//                         nodes.push(Node::Expression(expression));
//                     },
//                     _ => {
//                         unreachable!();
//                     },
//                 }
//             }
//         }
//         Ok(nodes)
//     }
//
//     pub fn parse_nodes(string: &str) -> std::result::Result<Vec<Node>, Error> {
//         let source_info = SourceInfo {
//             source: string.to_string(),
//             filename: None,
//         };
//         Ok(SimplelangSource::parse_nodes_with_source_info(string, &source_info)?)
//     }
// }
//
