use std::borrow::Cow;

use minilisp_util::{extend_lifetime, string_to_str};

use crate::{NodeInfo, NodePosition, SourceInfo};

pub fn stub_node_info<'a>(
    input: &'a str,
    start_pos: (usize, usize),
    end_pos: (usize, usize),
) -> NodeInfo<'a> {
    let node_info = NodeInfo {
        source: extend_lifetime!(
            &'a SourceInfo,
            &SourceInfo {
                source: string_to_str!(&input, 'a),
                filename: None,
            }
        ),
        name: None,
        input: string_to_str!(&input, 'a),
        start_pos: NodePosition::from_tuple(start_pos),
        end_pos: NodePosition::from_tuple(end_pos),
        inner: None,
    };
    node_info
}
pub fn stub_input<'a>(input: &'a str) -> (String, NodeInfo<'a>) {
    let node_info = stub_node_info(input, (1, 1), (1, input.len() + 1));
    (input.to_string(), node_info)
}
