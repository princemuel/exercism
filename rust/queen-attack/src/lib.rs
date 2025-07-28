#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        ((0..8).contains(&rank) && (0..8).contains(&file)).then_some(Self { rank, file })
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self { Queen { position } }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let dr = other.position.rank - self.position.rank;
        let dc = other.position.file - self.position.file;
        dr == 0 || dc == 0 || dr.abs() == dc.abs()
    }
}
