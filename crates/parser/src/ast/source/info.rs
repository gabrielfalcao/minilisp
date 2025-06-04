use pest::iterators::Pair;
use pest::Position;

use crate::{Rule, NodePosition};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct SourceInfo<'a> {
    pub source: &'a str,
    pub filename: Option<&'a str>,
}
impl<'a> SourceInfo<'a> {
    pub fn filename(&self) -> Option<String> {
        self.filename.clone().map(String::from)
    }
}
