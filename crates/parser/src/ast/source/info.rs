use std::borrow::Cow;

use pest::iterators::Pair;
use pest::Position;
use minilisp_util::string_to_str;
use crate::{NodePosition, Rule};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct SourceInfo<'a> {
    pub source: &'a str,
    pub filename: Option<&'a str>,
}
impl<'a> SourceInfo<'a> {
    pub fn new(source: &str, filename: Option<&str>) -> SourceInfo<'a> {
        SourceInfo {
            source: string_to_str!(source, 'a),
            filename: filename.map(|filename| string_to_str!(&filename, 'a))
        }
    }

    pub fn without_filename(source: &str) -> SourceInfo<'a> {
        SourceInfo {
            source: string_to_str!(source, 'a),
            filename: None,
        }
    }

    pub fn filename(&self) -> Option<String> {
        self.filename.clone().map(String::from)
    }
}

impl<'a> From<&str> for SourceInfo<'a> {
    fn from(source: &str) -> SourceInfo<'a> {
        SourceInfo::without_filename(source)
    }
}

impl<'a> From<String> for SourceInfo<'a> {
    fn from(source: String) -> SourceInfo<'a> {
        SourceInfo::without_filename(&source)
    }
}
