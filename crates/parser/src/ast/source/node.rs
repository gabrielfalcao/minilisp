use minilisp_util::{caller, dbg, try_result, unexpected, unwrap_result, with_caller};
use pest::iterators::Pair;
use pest::Position;

use crate::{NodePosition, Rule, SourceInfo};
use minilisp_util::string_to_str;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeInfo<'a> {
    pub input: &'a str,
    pub string: &'a str,
    pub start_pos: NodePosition,
    pub end_pos: NodePosition,
    pub source: SourceInfo<'a>,
}
impl<'a> std::fmt::Display for NodeInfo<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.string)
    }
}
impl<'a> std::fmt::Debug for NodeInfo<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "stub_node_info(&input, {:#?}, {:#?}, {:#?})",
            self.string,
            self.start_pos.to_tuple(),
            self.end_pos.to_tuple()
        )
    }
}

impl<'a> NodeInfo<'a> {
    pub fn string(&self) -> &'a str {
        self.string
    }

    pub fn input(&self) -> &'a str {
        self.input
    }

    pub fn filename(&self) -> Option<String> {
        self.source.filename()
    }

    pub fn with_string(&self, string: &'a str) -> NodeInfo<'a> {
        let mut info = self.clone();
        info.string = string;
        info
    }

    pub fn from_pair(pair: &'a Pair<Rule>, source_info: &'a SourceInfo) -> NodeInfo<'a> {
        let span = pair.as_span();
        let start_pos = NodePosition::from_pest(span.start_pos());
        let end_pos = NodePosition::from_pest(span.end_pos());
        NodeInfo {
            input: string_to_str!(&pair.get_input(), 'a),
            string: string_to_str!(&pair.to_string(), 'a),
            start_pos,
            end_pos,
            source: source_info.clone(),
        }
    }

    pub fn info(&self) -> NodeInfo<'a> {
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
                format!(
                    "\x1b[1;48;5;235m{}{}",
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
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}
