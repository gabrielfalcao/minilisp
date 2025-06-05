use std::borrow::Cow;

use minilisp_util::{
    caller, dbg, extend_lifetime, string_to_str, try_result, unexpected, unwrap_result, with_caller,
};
use pest::iterators::Pair;

use crate::{Rule, Value};

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum Item<'a> {
    List(Vec<Item<'a>>),
    Symbol(&'a str),
    Value(Value<'a>),
}

impl<'a> Item<'a> {
    pub fn from_pair(pair: &mut Pair<Rule>) -> Item<'a> {
        match pair.as_rule() {
            Rule::value => Item::Value(Value::from_pair(pair)),
            Rule::symbol => Item::Symbol(string_to_str!(pair.as_span().as_str(), 'a)),
            Rule::symbol_list =>
                Item::List(pair.clone().into_inner().map(|mut pair| Item::from_pair(&mut pair)).collect()),
            Rule::list => {
                let mut items = Vec::<Item<'a>>::new();
                let mut pairs = pair.clone().into_inner();
                pairs.next().expect("open_paren");
                loop {
                    if let Some(pair) = pairs.peek() {
                        if pair.as_rule() == Rule::close_paren {
                            break;
                        }
                    }
                    items.push(Item::from_pair(&mut pairs.next().expect("item")));
                }
                pairs.next().expect("close_paren");
                Item::List(items)
            },
            _ => unexpected!(pair),
        }
    }
}
