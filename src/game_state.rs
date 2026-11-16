#[derive(Clone, Copy, PartialEq, Eq)]

pub enum Symbol {
    Naught,
    Cross,
}

pub enum Outcome {
    Winner(Symbol),
    Tie,
}

#[derive(PartialEq, Eq)]
pub enum Cell {
    Taken(Symbol),
    Empty,
}

pub enum Error {
    CellAlreadyTaken,
    LocationOutOfBounds,
}

pub struct GameBoard {
    board: Vec<Vec<Cell>>,
    outcome: Option<Outcome>,
}

impl GameBoard {
    pub fn new() -> Self {
        let board = vec![
            vec![Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Empty],
            vec![Cell::Empty, Cell::Empty, Cell::Empty],
        ];

        return GameBoard {
            board,
            outcome: None,
        };
    }

    // loc must be 1-9
    pub fn place(&mut self, symbol: Symbol, loc: usize) -> Option<Error> {
        if loc > 9 {
            return Some(Error::LocationOutOfBounds);
        }
        let loc = loc - 1; // make loc zero indexed
        let cell = self.lookup_cell_mut(loc);

        if *cell != Cell::Empty {
            return Some(Error::CellAlreadyTaken);
        } else {
            *cell = Cell::Taken(symbol);
            self.outcome = self.check_outcome();
            return None;
        }
    }

    fn lookup_cell(&self, loc: usize) -> &Cell {
        let row = loc / 3;
        let col = loc % 3;
        return &self.board[row][col];
    }

    fn lookup_cell_mut(&mut self, loc: usize) -> &mut Cell {
        let row = loc / 3;
        let col = loc % 3;
        return &mut self.board[row][col];
    }

    fn check_outcome(&self) -> Option<Outcome> {
        let winner = self.check_for_winner();
        let tie = self.check_for_tie();

        winner.or(tie)
    }

    fn check_for_winner(&self) -> Option<Outcome> {
        return self
            .all_same_symbol(0, 1, 2) // first row
            .or(self.all_same_symbol(3, 4, 5)) // second row
            .or(self.all_same_symbol(6, 7, 8)) // third row
            .or(self.all_same_symbol(0, 3, 6)) // first column
            .or(self.all_same_symbol(1, 4, 7)) // second column
            .or(self.all_same_symbol(2, 5, 8)) // third column
            .or(self.all_same_symbol(0, 4, 8)) // diagonal top left to bottom right
            .or(self.all_same_symbol(6, 4, 2)) // diagonal bottom left to top right
            .map(|o| Outcome::Winner(o));
    }

    fn check_for_tie(&self) -> Option<Outcome> {
        for v in &self.board {
            for c in v {
                if *c == Cell::Empty {
                    return None;
                }
            }
        }
        return Some(Outcome::Tie);
    }

    fn all_same_symbol(&self, one: usize, two: usize, three: usize) -> Option<Symbol> {
        let cell_one = self.lookup_cell(one);
        let cell_two = self.lookup_cell(two);
        let cell_three = self.lookup_cell(three);

        match cell_one {
            Cell::Empty => return None,
            Cell::Taken(symbol) => {
                if *cell_two == *cell_one && *cell_three == *cell_one {
                    return Some(*symbol);
                } else {
                    return None;
                }
            }
        }
    }
}
