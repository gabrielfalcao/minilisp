use std::borrow::{Borrow, Cow};

use minilisp_util::{
    caller, dbg, extend_lifetime, try_result, unexpected, unwrap_result, with_caller,
};
use pest::error::LineColLocation;
use pest::iterators::Pair;
use pest::{Position, RuleType};

use crate::{Error, NodePosition, Rule, Source};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Node<'a> {
    pub input: Cow<'a, str>,
    pub name: Option<String>,
    pub start_pos: NodePosition,
    pub end_pos: NodePosition,
    pub source: &'a Source<'a>,
    pub inner: Option<Vec<Node<'a>>>,
}
impl<'a> std::fmt::Display for Node<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            [
                self.input.to_string(),
                if let Some(nodes) = &self.inner {
                    format!(
                        ", [{}]",
                        nodes
                            .iter()
                            .map(|node| node.to_string())
                            .collect::<Vec<String>>()
                            .join(", ")
                    )
                } else {
                    String::new()
                }
            ]
            .join("")
            .trim()
        )
    }
}
impl<'a> std::fmt::Debug for Node<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            [
                self.name.clone().map(String::from).unwrap_or_else(|| "Node".to_string()),
                " {".to_string(),
                [
                    format!("input: '{}'", self.input),
                    if let Some(nodes) = &self.inner {
                        format!("nodes: {:#?}", &nodes)
                    } else {
                        String::new()
                    }
                ]
                .iter()
                .filter(|string| string.len() > 0)
                .map(String::from)
                .collect::<Vec<String>>()
                .join(", "),
                "}".to_string(),
            ]
            .join("")
        )
    }
}

impl<'a> Node<'a> {
    pub fn input(&'a self) -> &'a str {
        self.input.borrow()
    }

    pub fn inner(&self) -> Vec<Node<'a>> {
        if let Some(nodes) = &self.inner {
            nodes.clone()
        } else {
            Vec::new()
        }
    }

    pub fn filename(&self) -> Option<String> {
        self.source.filename()
    }

    pub fn with_input(&self, input: &'a str) -> Node<'a> {
        let mut info = self.clone();
        info.input = Cow::from(input);
        info
    }

    pub fn from_pair(pair: &'a Pair<Rule>, source: &'a Source) -> Node<'a> {
        let span = pair.as_span();
        let start_pos = NodePosition::from_pest(span.start_pos());
        let end_pos = NodePosition::from_pest(span.end_pos());

        Node {
            input: Cow::from(span.as_str()),
            name: Some(format!("{:#?}", pair.as_rule())),
            // string: Cow::from(&pair.to_string()),
            start_pos,
            end_pos,
            source,
            inner: {
                let inner = pair.clone().into_inner();
                if inner.peek().is_none() {
                    None
                } else {
                    Some(
                        inner
                            .map(|pair| {
                                Node::from_pair(
                                    extend_lifetime!(&'a Pair<Rule>, &pair),
                                    extend_lifetime!(&'a Source, source),
                                )
                            })
                            .collect(),
                    )
                }
            },
        }
    }

    pub fn from_error(error: pest::error::Error<Rule>, source: &'a Source<'a>) -> Node<'a> {
        let (start_pos, end_pos) = match error.line_col.clone() {
            LineColLocation::Pos(line_col) => (
                NodePosition::from_tuple(line_col.clone()),
                NodePosition::from_tuple(line_col.clone()),
            ),
            LineColLocation::Span(start_pos, end_pos) =>
                (NodePosition::from_tuple(start_pos), NodePosition::from_tuple(end_pos)),
        };
        Node {
            input: Cow::from(error.line().to_string()),
            name: None,
            start_pos,
            end_pos,
            source,
            inner: None,
        }
    }

    pub fn info(&self) -> Node<'a> {
        self.clone()
    }

    pub fn start_pos(&self) -> (usize, usize) {
        self.start_pos.to_tuple()
    }

    pub fn end_pos(&self) -> (usize, usize) {
        self.end_pos.to_tuple()
    }

    pub fn highlight_input(&self, indent: usize) -> String {
        minilisp_util::color::fore(self.highlight_input_chars(indent), 32)
    }

    fn highlight_input_chars(&self, indent: usize) -> String {
        let start_pos = self.start_pos.clone();
        let end_pos = self.end_pos.clone();
        self.input
            .lines()
            .enumerate()
            .map(|(no, line)| {
                (
                    no + 1,
                    line.chars()
                        .enumerate()
                        .map(|(no, column)| (no + 1, column.to_string()))
                        .collect::<Vec<(usize, String)>>(),
                )
            })
            .map(|(line, columns)| {
                minilisp_util::color::bg(
                    format!(
                        "{}{}",
                        " ".repeat(indent),
                        columns
                            .iter()
                            .map(|(column, text)| {
                                let column = column.clone();
                                if line == start_pos.line && column == start_pos.column {
                                    minilisp_util::color::bgfg(text, 235, 198)
                                } else if line == end_pos.line && column == end_pos.column {
                                    [
                                        minilisp_util::color::reset(""),
                                        minilisp_util::color::bg(text, 235),
                                    ]
                                    .join("")
                                } else {
                                    minilisp_util::color::bg(text, 235)
                                }
                            })
                            .collect::<String>()
                    ),
                    235,
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}
