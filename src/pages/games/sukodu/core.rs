use std::array;

use yew::prelude::*;

pub struct Square {
    pub rows: Vec<Vec<usize>>,
}

impl Square {
    pub fn new(rows: Vec<Vec<usize>>) -> Self {
        Self { rows }
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
