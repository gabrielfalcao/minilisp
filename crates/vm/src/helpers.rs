use minilisp_data_structures::{Cell, Symbol, Value};
use minilisp_util::{dbg, try_result};

use crate::{with_caller, Error, ErrorType, Result};

pub fn runtime_error(message: String, previous: Option<Error>) -> Error {
    with_caller!(Error::with_previous_error(
        message,
        ErrorType::RuntimeError,
        previous
    ))
}
