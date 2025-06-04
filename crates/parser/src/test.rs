use crate::{NodeInfo, NodePosition, SourceInfo};
use minilisp_util::string_to_str;

pub fn stub_node_info<'a>(
    input: &'a str,
    start_pos: (usize, usize),
    end_pos: (usize, usize),
) -> NodeInfo<'a> {
    let node_info = NodeInfo {
        source: SourceInfo {
            source: string_to_str!(input, 'a),
            filename: None,
        },
        input: string_to_str!(input, 'a),
        start_pos: NodePosition::from_tuple(start_pos),
        end_pos: NodePosition::from_tuple(end_pos),
    };
    node_info
}
pub fn stub_input<'a>(input: &'a str) -> (String, NodeInfo<'a>) {
    let node_info = stub_node_info(input, (1, 1), (1, input.len() + 1));
    (input.to_string(), node_info)
}
