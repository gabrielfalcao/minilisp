use std::borrow::Cow;
use std::str::FromStr;

use minilisp_util::{
    caller, dbg, extend_lifetime, string_to_str, try_result, unexpected, unwrap_result, with_caller,
};
use pest::iterators::Pair;

use crate::Rule;
#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub enum Value<'a> {
    Float(f64),
    Integer(i64),
    String(&'a str),
    // String(Cow<'a, str>),
    UnsignedInteger(u64),
    T,
    #[default]
    Nil,
}

impl<'a> Value<'a> {
    pub fn from_pair(pair: &mut Pair<Rule>) -> Value<'a> {
        match pair.as_rule() {
            Rule::float => Value::Float(f64::from_str(pair.as_span().as_str()).expect("float")),
            Rule::integer => Value::Integer(i64::from_str(pair.as_span().as_str()).expect("integer")),
            Rule::string => Value::String(string_to_str!(pair.as_span().as_str(), 'a)),
            Rule::t => Value::T,
            Rule::unsigned =>
                Value::UnsignedInteger(u64::from_str(pair.as_span().as_str()).expect("unsigned integer")),
            Rule::value => Value::from_pair(&mut pair.clone().into_inner().next().expect("value")),
            Rule::nil => Value::Nil,
            _ => unexpected!(pair),
        }
    }
}
