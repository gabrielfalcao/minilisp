#![allow(unused)]
#![feature(intra_doc_pointers)]
pub mod traits;
pub use traits::ListValue;
pub mod cons;
pub use cons::{car, cdr, cons};
pub mod cell;
pub use cell::Cell;
// pub mod list;
// pub use list::List;
pub mod value;
pub use value::Value;
pub mod node;
pub use minilisp_util::color;
pub use node::{subtree_delete, Node};

pub mod macros;
pub mod unique_pointer;
pub use unique_pointer::UniquePointer;
pub mod refcounter;
pub use refcounter::RefCounter;
pub(crate) mod internal;
pub mod test;
