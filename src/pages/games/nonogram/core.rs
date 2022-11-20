use std::ops::{Index, IndexMut};

#[derive(Clone, PartialEq)]
pub struct Rule {
    pub set: Vec<usize>,
}

impl Rule {
    pub fn len(&self) -> usize {
        self.set.len()
    }
    pub fn is_full_and_valid(&self, row: Vec<bool>) -> bool {
        // Do not check previous blocks
        let mut index = 0;

        // All the rules are satisfied
        for r in self.set.clone() {
            let mut count = 0;
            for i in index..row.len() {
                index = i;
                if row[i] {
                    count += 1;
                } else {
                    if count == r {
                        break;
                    }
                    if count > 0 {
                        return false;
                    }
                }
            }
            if count != r {
                return false;
            }
        }
        // no rule left --> there must be no black square left
        if row.iter().skip(index + 1).any(|el| *el) {
            return false;
        }
        true
    }
}

impl Index<usize> for Rule {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.set[index]
    }
}

#[macro_export]
macro_rules! rule {
    ($($x:expr),*) => {
        Rule{
            set: vec![$($x),*]
        }
    };
}

#[derive(Clone, PartialEq)]
pub struct NonogramCore {
    pub grid: Vec<Vec<bool>>,
    pub rotated: Vec<Vec<bool>>,
}

impl NonogramCore {
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut grid = Vec::new();
        for _ in 0..rows {
            let mut row = Vec::new();
            for _ in 0..cols {
                row.push(false);
            }
            grid.push(row);
        }
        let mut rotaded_grid = Vec::new();
        for _ in 0..cols {
            let mut row = Vec::new();
            for _ in 0..rows {
                row.push(false);
            }
            rotaded_grid.push(row);
        }
        Self { grid, rotated: rotaded_grid }
    }

    pub fn change_cell(&self, row: usize, col: usize) -> Self {
        let mut clone = self.clone();
        clone.grid[row][col] = !self.grid[row][col];
        clone.rotated[col][row] = !self.rotated[col][row];
        clone
    }
}

impl Index<usize> for NonogramCore {
    type Output = Vec<bool>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.grid[index]
    }
}

impl IndexMut<usize> for NonogramCore {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.grid[index]
    }
}
