#[derive(Debug)]
pub struct ChessPosition {
    rank: u8,
    file: u8,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        ((0..8).contains(&rank) && (0..8).contains(&file)).then_some(Self {
            rank: rank as u8,
            file: file as u8,
        })
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self { Queen { position } }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let dr = (other.position.rank as i8) - (self.position.rank as i8);
        let dc = (other.position.file as i8) - (self.position.file as i8);

        if dr == 0 && dc == 0 {
            return false;
        }

        dr == 0 || dc == 0 || dr.abs() == dc.abs()
    }
}
