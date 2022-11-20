use std::ops::Index;

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

struct NonogramCore {
    pub rows: usize,
    pub cols: usize,
    pub rows_rules: Vec<Vec<usize>>,
    pub cols_rules: Vec<Vec<usize>>,
    pub grid: Vec<Vec<bool>>,
}

impl NonogramCore {
    pub fn new(
        rows: usize,
        cols: usize,
        rows_rules: Vec<Vec<usize>>,
        cols_rules: Vec<Vec<usize>>,
    ) -> Self {
        let mut grid = Vec::new();
        for _ in 0..rows {
            let mut row = Vec::new();
            for _ in 0..cols {
                row.push(false);
            }
            grid.push(row);
        }
        Self {
            rows,
            cols,
            rows_rules,
            cols_rules,
            grid,
        }
    }
}
