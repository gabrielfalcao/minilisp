use std::borrow::Cow;
use std::collections::VecDeque;

use minilisp_util::{
    caller, dbg, extend_lifetime, string_to_str, try_result, unexpected, unwrap_result, with_caller,
};
use pest::iterators::Pair;

use crate::{Rule, Value};

#[derive(Clone, PartialEq, PartialOrd, Debug, Hash)]
pub enum Item<'a> {
    List(VecDeque<Item<'a>>),
    Symbol(&'a str),
    Value(Value<'a>),
}

impl<'a> Item<'a> {
    pub fn as_list(&self) -> Option<VecDeque<Item<'a>>> {
        if let Item::List(items) = self {
            Some(items.clone())
        } else {
            None
        }
    }

    pub fn as_symbol<'c>(&self) -> Option<&'c str> {
        if let Item::Symbol(symbol) = self {
            Some(string_to_str!(symbol, 'c))
        } else {
            None
        }
    }
    pub fn as_value<'c>(&self) -> Option<&'c str> {
        if let Item::Symbol(symbol) = self {
            Some(string_to_str!(symbol, 'c))
        } else {
            None
        }
    }
}
impl<'a> Item<'a> {
    pub fn from_pair(pair: &mut Pair<Rule>) -> Item<'a> {
        match pair.as_rule() {
            Rule::value => Item::Value(Value::from_pair(pair)),
            Rule::symbol => Item::Symbol(string_to_str!(&pair.as_span().as_str(), 'a)),
            Rule::item => Item::from_pair(&mut pair.clone().into_inner().next().expect("item")),
            Rule::list | Rule::symbol_list => {
                let mut items = VecDeque::<Item<'a>>::new();
                let mut pairs = pair.clone().into_inner();
                loop {
                    if let Some(pair) = pairs.peek() {
                        if pair.as_rule() == Rule::close_paren {
                            break;
                        }
                    }
                    let mut pair = pairs.next().expect("quote, open_paren or item");
                    match pair.as_rule() {
                        Rule::quote => {
                            items.push_back(Item::Symbol("list"));
                        },
                        Rule::open_paren => continue,
                        Rule::value | Rule::symbol | Rule::item => {
                            items.push_back(Item::from_pair(&mut pair));
                            continue;
                        },
                        _ => {
                            unexpected!(pair);
                        },
                    }
                }
                pairs.next().expect("close_paren");
                Item::List(items)
            },
            _ => unexpected!(pair),
        }
    }
}
