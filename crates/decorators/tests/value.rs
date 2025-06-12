use minilisp_decorators::ast;
#[ast]
pub enum Value {
    String(String),
    Integer(i8),
}
