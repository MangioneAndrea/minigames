#[derive(Clone)]
pub struct Cell {
    pub value: u8,
    pub is_given: bool,
    pub is_wrong_square: bool,
    pub is_wrong_line: bool,
}

impl Cell {
    pub fn new(value: u8) -> Self {
        Self {
            value,
            is_given: value != 0,
            is_wrong_square: false,
            is_wrong_line: false,
        }
    }

    pub fn format(&self) -> String {
        let mut format = String::new();

        if self.is_given {
            format.push_str("bg-gray-100 ");
            if self.is_wrong_square || self.is_wrong_line {
                format.push_str("bg-red-200 ");
            }
        } else {
            format.push_str("bg-white ");
            if self.is_wrong_line || self.is_wrong_square {
                format.push_str("bg-red-100 ");
            }
        }

        format
    }

    pub fn increment(&mut self) {
        if self.is_given {
            return;
        }
        if self.value < 9 {
            self.value += 1;
        } else {
            self.value = 0;
        }
    }
}

#[derive(Clone)]
pub struct Square {
    pub rows: Vec<Vec<Cell>>,
}

impl Square {
    pub fn new(rows: Vec<Vec<u8>>) -> Self {
        Self {
            rows: rows
                .into_iter()
                .map(|row| row.into_iter().map(Cell::new).collect())
                .collect(),
        }
    }

    pub fn increase_at(&mut self, r: usize, c: usize) {
        self.rows[r][c].increment();
    }

    pub fn check_and_mark(&mut self) {
        for row in self.rows.iter_mut() {
            for cell in row.iter_mut() {
                cell.is_wrong_square = false;
            }
        }

        for r in 0..3 {
            for c in 0..3 {
                for r2 in 0..3 {
                    for c2 in 0..3 {
                        if r2 != r || c2 != c {
                            if self.rows[r2][c2].value == self.rows[r][c].value
                                && self.rows[r][c].value != 0
                            {
                                self.rows[r][c].is_wrong_square = true;
                                self.rows[r2][c2].is_wrong_square = true;
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct Grid {
    pub squares: Vec<Vec<Square>>,
}

impl Grid {
    pub fn new(squares: Vec<Vec<Square>>) -> Self {
        Self { squares }
    }

    pub fn check_and_mark(&mut self, row: usize, column: usize) {
        for index in 0..9 {
            self.squares[index / 3][column / 3].rows[index % 3][column % 3].is_wrong_line = false;
            self.squares[row / 3][index / 3].rows[row % 3][index % 3].is_wrong_line = false;
        }

        for i in 0..9 {
            // mark as false if the value is double in the row
            if self.squares[i / 3][column / 3].rows[i % 3][column % 3].value
                == self.squares[row / 3][column / 3].rows[row % 3][column % 3].value
                && self.squares[row / 3][column / 3].rows[row % 3][column % 3].value != 0
                && i != row
            {
                self.squares[i / 3][column / 3].rows[i % 3][column % 3].is_wrong_line = true;
                self.squares[row / 3][column / 3].rows[row % 3][column % 3].is_wrong_line = true;
            }
            // mark as false if the value is double in the column
            if self.squares[row / 3][i / 3].rows[row % 3][i % 3].value
                == self.squares[row / 3][column / 3].rows[row % 3][column % 3].value
                && self.squares[row / 3][column / 3].rows[row % 3][column % 3].value != 0
                && i != column
            {
                self.squares[row / 3][i / 3].rows[row % 3][i % 3].is_wrong_line = true;
                self.squares[row / 3][column / 3].rows[row % 3][column % 3].is_wrong_line = true;
            }
        }
    }
}
