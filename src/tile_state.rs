#[derive(Clone, PartialEq, Default)]
pub struct TileState {
    pub cell_type: TileType,
    pub is_open: bool,
    pub number: usize,
    pub flagged: bool,
}

#[derive(Clone, PartialEq)]
pub enum TileType {
    Number { local_bombs: usize },
    Bomb,
}

impl Default for TileType {
    fn default() -> Self {
        TileType::Number { local_bombs: 0 }
    }
}
