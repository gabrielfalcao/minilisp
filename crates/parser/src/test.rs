use std::borrow::Cow;

use crate::{Source, Span, SpanPosition};

pub fn stub_span_info<'a>(
    input: &'a str,
    start_pos: (usize, usize),
    end_pos: (usize, usize),
) -> Span<'a> {
    let span_info = Span {
        source: Source {
            source: Cow::from(input),
            filename: None,
        },
        name: None,
        input: Cow::from(input),
        start_pos: SpanPosition::from_tuple(start_pos),
        end_pos: SpanPosition::from_tuple(end_pos),
        inner: None,
    };
    span_info
}
pub fn stub_input<'a>(input: &'a str) -> (String, Span<'a>) {
    let span_info = stub_span_info(input, (1, 1), (1, input.len() + 1));
    (input.to_string(), span_info)
}
