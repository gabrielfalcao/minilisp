use pest::Position;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct NodePosition {
    pub line: usize,
    pub column: usize,
}

impl NodePosition {
    pub fn from_tuple(line_col: (usize, usize)) -> NodePosition {
        let (line, column) = line_col;
        NodePosition { line, column }
    }

    pub fn from_pest(position: Position) -> NodePosition {
        let (line, column) = position.line_col();
        NodePosition { line, column }
    }

    pub fn to_tuple(&self) -> (usize, usize) {
        (self.line, self.column)
    }
}
