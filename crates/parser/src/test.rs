use std::borrow::Cow;

use minilisp_util::{extend_lifetime, string_to_str};

use crate::{Node, NodePosition, Source};

pub fn stub_node_info<'a>(
    input: &'a str,
    start_pos: (usize, usize),
    end_pos: (usize, usize),
) -> Node<'a> {
    let node_info = Node {
        source: extend_lifetime!(
            &'a Source,
            &Source {
                source: Cow::from(input),
                filename: None,
            }
        ),
        name: None,
        input: Cow::from(input),
        start_pos: NodePosition::from_tuple(start_pos),
        end_pos: NodePosition::from_tuple(end_pos),
        inner: None,
    };
    node_info
}
pub fn stub_input<'a>(input: &'a str) -> (String, Node<'a>) {
    let node_info = stub_node_info(input, (1, 1), (1, input.len() + 1));
    (input.to_string(), node_info)
}
