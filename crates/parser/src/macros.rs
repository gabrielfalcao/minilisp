#[macro_export]
macro_rules! assert_ast_equal {
    ($left:expr, $right:expr) => {{
        k9::assert_equal!($left.clone().iter().map(|node|node.as_item()).collect::<std::collections::VecDeque<Item<'static>>>(), $right);
    }};
}
