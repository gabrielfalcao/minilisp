use pest::iterators::{Pair, Pairs};
use pest::{Position, Span};
use minilisp_util::{caller, dbg, try_result, unexpected, unwrap_result, with_caller};

use crate::{Error, NodeInfo, Result, Rule, SourceInfo};

pub fn format_span(span: &Span) -> String {
    format!(
        "from position {} to {}",
        format_position(&span.start_pos()),
        format_position(&span.end_pos())
    )
}
pub fn format_position(pos: &Position) -> String {
    let (line, column) = pos.line_col();
    format!("[{}:{}]", line, column)
}

pub fn format_rule(pair: &Pair<Rule>) -> String {
    format!("{:#?} {}", pair.as_rule(), format_span(&pair.as_span()))
}

// pub fn parse_error_expecting<'a, T: std::fmt::Display>(
//     expecting: T,
//     pair: &'a Pair<Rule>,
//     source_info: &'a SourceInfo,
// ) -> Error<'a> {
//     with_caller!(Error::new(
//         format!("expected {}, found {}", expecting, rule_to_string(&pair.as_rule())),
//         NodeInfo::from_pair(pair, source_info),
//     ))
// }
// pub fn parse_error_expecting_any_of<'a, T: std::fmt::Display>(
//     expecting: T,
//     parent_pair: &'a Pair<Rule>,
//     pairs: &'a Pairs<Rule>,
//     source_info: &'a SourceInfo,
// ) -> Error<'a> {
//     with_caller!(Error::new(
//         format!(
//             "expected {} found {}",
//             expecting,
//             rule_options_to_string(pairs.clone().map(|pair| pair.as_rule()).collect::<Vec<Rule>>())
//         ),
//         NodeInfo::from_pair(parent_pair, source_info),
//     ))
// }
// pub fn expect_single_inner_pair<'a>(
//     expected_rules: Vec<Rule>,
//     pair: &'a Pair<Rule>,
//     source_info: &'a SourceInfo,
// ) -> Result<'a, Pair<'a, Rule>> {
//     match pair.clone().into_inner().next() {
//         Some(pair) => Ok(try_result!(expect_rules(expected_rules, &pair, &source_info))),
//         None => Err(with_caller!(Error::new(
//             format!(
//                 "no inner pairs in {:#?} when expecting {}",
//                 pair.as_str(),
//                 rule_options_to_string(expected_rules)
//             ),
//             NodeInfo::from_pair(pair, source_info),
//         ))),
//     }
// }

// pub fn expect_next_pair_single_rule<'a>(
//     parent_pair: &'a Pair<Rule>,
//     pairs: &'a mut Pairs<Rule>,
//     expected_rule: Rule,
//     source_info: &SourceInfo,
// ) -> Result<'a, Pair<'a, Rule>> {
//     match pairs.next() {
//         Some(pair) => Ok(expect_rule(expected_rule, &pair, &source_info)?),
//         None => Err(with_caller!(parse_error_expecting_any_of(
//             rule_to_string(&expected_rule),
//             parent_pair,
//             pairs,
//             &source_info
//         ))),
//     }
// }
// pub fn expect_next_pair<'a>(
//     parent_pair: &'a Pair<Rule>,
//     pairs: &'a mut Pairs<Rule>,
//     expected_rules: Vec<Rule>,
//     source_info: &'a SourceInfo,
// ) -> Result<'a, (Pair<'a, Rule>, Option<Pair<'a, Rule>>)> {
//     match pairs.next() {
//         Some(pair) =>
//             Ok((try_result!(expect_rules(expected_rules, &pair, &source_info)), pairs.peek())),
//         None => Err(with_caller!(parse_error_expecting_any_of(
//             rule_options_to_string(expected_rules),
//             parent_pair,
//             pairs,
//             source_info
//         ))),
//     }
// }

// pub fn expect_rules<'a>(
//     rules: Vec<Rule>,
//     pair: &'a Pair<'a, Rule>,
//     source_info: &'a SourceInfo,
// ) -> Result<'a, Pair<'a, Rule>> {
//     if !rules.iter().any(|rule| pair.as_rule() == rule.clone()) {
//         return Err(with_caller!(parse_error_expecting(
//             rule_options_to_string(rules),
//             &pair,
//             source_info
//         )));
//     }
//     Ok(pair.clone())
// }

// pub fn expect_rule<'a>(
//     rule: Rule,
//     pair: &Pair<'a, Rule>,
//     source_info: &'a SourceInfo,
// ) -> Result<'a, Pair<'a, Rule>> {
//     if pair.as_rule() != rule {
//         return Err(with_caller!(parse_error_expecting(rule_to_string(&rule), &pair, source_info)));
//     }
//     Ok(pair.clone())
// }

// pub fn rule_to_string(rule: &Rule) -> String {
//     format!(
//         "{:#?}",
//         match *rule {
//             Rule::EOI => "end of input".to_string(),
//             Rule::WHITESPACE => "whitespace".to_string(),
//             _ => format!("{:#?}", rule).replace("_", " "),
//         }
//     )
// }
// pub fn rule_options_to_string(rules: Vec<Rule>) -> String {
//     let mut rules = rules.iter().map(|rule| rule_to_string(rule)).collect::<Vec<String>>();
//     match rules.len() {
//         0 => String::new(),
//         1 => rules.join(""),
//         2 => rules.join(" or "),
//         _ => {
//             let last = rules.pop().unwrap();
//             format!("{} or {}", rules.join(", "), last)
//         },
//     }
// }
