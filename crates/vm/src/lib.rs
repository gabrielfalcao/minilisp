#![allow(
    unused,
    mutable_transmutes
)]
#![feature(trait_alias)]

pub use errors::{Error, ErrorType, Result};
pub mod builtin;
pub mod errors;
pub mod helpers;
pub mod macros;
pub mod sym;
pub(crate) mod symbol_table;
pub mod test;
pub use sym::Sym;
pub mod function;
pub use builtin::BuiltinFunction;
pub use function::Function;
pub use helpers::runtime_error;
pub mod virtual_machine;
pub use virtual_machine::VirtualMachine;
pub mod context;
pub use context::Context;
