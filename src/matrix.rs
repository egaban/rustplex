use std::fmt;

pub(crate) struct Matrix {
    values: Vec<Vec<f64>>,
    num_rows: usize,
    num_cols: usize,
}

impl Matrix {
    pub fn new() -> Matrix {
        Matrix {
            values: Vec::new(),
            num_rows: 0,
            num_cols: 0,
        }
    }

    /// Adds a new row and returns its index.
    pub fn add_row(&mut self) -> usize {
        let new_row = vec![0.0; self.num_cols];
        self.values.push(new_row);
        self.num_rows += 1;
        self.num_rows - 1
    }

    /// Adds a new column and returns its index.
    pub fn add_column(&mut self) -> usize {
        for row in self.values.iter_mut() {
            row.push(0.0);
        }
        self.num_cols += 1;
        self.num_cols - 1
    }

    pub fn set_value(&mut self, row: usize, column: usize, value: f64) {
        assert!(row < self.num_rows);
        assert!(column < self.num_cols);
        self.values[row][column] = value;
    }

    pub fn get_value(&self, row: usize, column: usize) -> f64 {
        assert!(row < self.num_rows);
        assert!(column < self.num_cols);
        self.values[row][column]
    }

    pub fn num_cols(&self) -> usize {
        self.num_cols
    }

    pub fn num_rows(&self) -> usize {
        self.num_rows
    }
}

impl fmt::Debug for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.values {
            for value in row {
                write!(f, "{:.2}\t", value)?;
            }
            writeln!(f, "")?;
        }
        fmt::Result::Ok(())
    }
}
