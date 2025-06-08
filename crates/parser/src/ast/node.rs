use std::borrow::Cow;
use std::collections::VecDeque;

use minilisp_util::{extend_lifetime, unexpected};
use pest::iterators::Pair;

use crate::{Item, Rule, Source, Span, Value};

#[derive(Clone, PartialEq, PartialOrd, Debug, Hash)]
pub enum Node<'a> {
    List(Span<'a>, VecDeque<Node<'a>>),
    Symbol(Span<'a>, Cow<'a, str>),
    Value(Span<'a>, Value<'a>),
}

impl<'a> Node<'a> {
    pub fn as_item<'c>(&self) -> Item<'a> {
        match self {
            Node::List(_span, sexpr) => Item::List(sexpr.iter().map(|node| node.as_item()).collect()),
            Node::Symbol(_span, symbol) => Item::Symbol(symbol.clone()),
            Node::Value(_span, value) => Item::Value(value.clone()),
        }
    }
}
impl<'a> Node<'a> {
    pub fn from_pair(pair: &'a Pair<'a, Rule>, source: &'a Source) -> Node<'a> {
        let span = Span::from_pair(extend_lifetime!(&'a Pair<'a, Rule>, &pair), extend_lifetime!(&'a Source, &source));
        match pair.as_rule() {
            Rule::value => Node::Value(span.clone(), Value::from_pair(extend_lifetime!(&'a Pair<'a, Rule>, &pair))),
            Rule::symbol => Node::Symbol(span.clone(), Cow::from(span.input().to_string())),
            Rule::item => {
                let pair = pair.clone().into_inner().next().expect("item");
                Node::from_pair(extend_lifetime!(&'a Pair<'a, Rule>, &pair), extend_lifetime!(&'a Source, &source))
            },
            Rule::sexpr => {
                let mut items = VecDeque::<Node<'a>>::new();
                let mut pairs = pair.clone().into_inner();
                loop {
                    if let Some(pair) = pairs.peek() {
                        if pair.as_rule() == Rule::close_paren {
                            break;
                        }
                    }
                    let pair = pairs.next().expect("quote, open_paren or item");
                    match pair.as_rule() {
                        Rule::quote => {
                            items.push_back(Node::Symbol(span.clone(), Cow::from("sexpr")));
                        },
                        Rule::open_paren => continue,
                        Rule::value | Rule::symbol | Rule::item => {
                            items.push_back(Node::from_pair(extend_lifetime!(&'a Pair<'a, Rule>, &pair), extend_lifetime!(&'a Source, &source)));
                            continue;
                        },
                        _ => {
                            unexpected!(pair);
                        },
                    }
                }
                pairs.next().expect("close_paren");
                Node::List(span.clone(), items)
            },
            _ => unexpected!(pair),
        }
    }
}
