#[derive(Clone, PartialEq, Default)]
pub struct CellState {
    pub cell_type: CellType,
    pub is_open: bool,
    pub number: usize,
    pub flagged: bool,
}

#[derive(Clone, PartialEq)]
pub enum CellType {
    Number { local_bombs: usize },
    Bomb,
}

impl Default for CellType {
    fn default() -> Self {
        CellType::Number { local_bombs: 0 }
    }
}
