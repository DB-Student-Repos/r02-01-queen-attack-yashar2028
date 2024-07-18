#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        // Validate that the position is within the bounds of a standard chessboard (1 to 8)
        if (0..8).contains(&rank) && (0..8).contains(&file) {
            Some(ChessPosition { rank, file })
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen(position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let self_pos = &self.0;
        let other_pos = &other.0;

        // Same rank
        if self_pos.rank == other_pos.rank {
            return true;
        }

        // Same file
        if self_pos.file == other_pos.file {
            return true;
        }

        // Same diagonal
        if (self_pos.rank - other_pos.rank).abs() == (self_pos.file - other_pos.file).abs() {
            return true;
        }

        // Otherwise, they cannot attack each other
        false
    }
}
