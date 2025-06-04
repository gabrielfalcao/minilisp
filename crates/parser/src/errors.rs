use std::fmt::{Debug, Display, Formatter};

pub use minilisp_util::{with_caller, Caller, Traceback};

use crate::ast::{NodeInfo, NodePosition, SourceInfo};

#[derive(Clone, PartialEq, Eq)]
pub struct Error<'a> {
    message: String,
    info: NodeInfo<'a>,
    callers: Vec<Caller>,
}
impl<'a> Error<'a> {
    pub fn new<T: Display>(message: T, info: NodeInfo<'a>) -> Self {
        let message = message.to_string();
        Error {
            message: message,
            info,
            callers: Vec::new(),
        }
    }
}
impl std::error::Error for Error<'_> {}
impl<'a> Traceback for Error<'a> {
    fn message(&self) -> String {
        self.message.to_string()
    }

    fn with(&self, caller: Caller) -> Self {
        let mut error = self.clone();
        error.callers.insert(0, caller);
        error
    }

    fn callers(&self) -> Vec<Caller> {
        self.callers.to_vec()
    }
}
impl<'a> Display for Error<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "\x1b[0m{}\n\n\x1b[1;48;5;198m\x1b[1;38;5;235mreason:\x1b[0m {}",
            self.info.highlight_input(4),
            self.highlight_message(),
        )
    }
}
impl<'a> Debug for Error<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "\x1b[1;38;5;202min source:\n{}\n\n\x1b[1;38;5;220mStacktrace:\n{}\n",
            self.to_string(),
            self.callers_to_string(4)
        )
    }
}

pub type Result<'a, T> = std::result::Result<T, Error<'a>>;
