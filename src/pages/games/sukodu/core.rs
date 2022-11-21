pub struct Cell {
    pub value: u8,
    pub is_given: bool,
}

impl Cell {
    pub fn new(value: u8) -> Self {
        Self {
            value,
            is_given: value != 0,
        }
    }

    pub fn format(&self) -> String{
        if self.is_given {
            "bg-gray-100".to_string()
        } else {
            "bg-white".to_string()
        }
    }
}

pub struct Square {
    pub rows: Vec<Vec<Cell>>,
}

impl Square {
    pub fn new(rows: Vec<Vec<u8>>) -> Self {
        Self { rows: rows.into_iter().map(|row| row.into_iter().map(Cell::new).collect()).collect() }
    }
}

pub struct Grid {
    pub squares: Vec<Vec<Square>>,
}

impl Grid {
    pub fn new(squares: Vec<Vec<Square>>) -> Self {
        Self { squares }
    }
}
