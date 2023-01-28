use crate::matrix::Matrix;
use crate::{Constraint, ConstraintType, Model};

use std::collections::HashMap;

const RHS_INDEX: usize = 0;
const Z_INDEX: usize = 1;

enum ColumnType {
    Single(usize),
}

pub struct Solver<'a> {
    tableau: Matrix,
    model: &'a Model,
    variable_column: HashMap<String, ColumnType>,
    basic_variable: Vec<usize>,
}

impl<'a> Solver<'a> {
    pub fn new(model: &'a Model) -> Solver {
        log::debug!("Creating a simplex solver");
        let mut solver = Solver {
            tableau: Matrix::new(),
            model,
            variable_column: HashMap::new(),
            basic_variable: Vec::new(),
        };

        solver.initialize_tableau();
        return solver;
    }

    pub fn solve(&mut self) {
        log::info!("Starting to solve simplex model");

        while self.should_continue() {
            let column = self.choose_pivot_column();
            let row = self.choose_pivot_row(column);
            self.pivot(row, column);
        }
        log::info!("Final value = {}", self.get_objective_value());
    }

    fn initialize_tableau(&mut self) {
        self.create_row0();
        for (_, constraint) in &self.model.constraints {
            self.create_constraint(constraint);
        }
    }

    /// Creates the first row of the tableau, containing the reduced costs.
    fn create_row0(&mut self) {
        self.tableau.add_row();
        self.tableau.add_column();
        self.tableau.add_column();

        self.tableau.set_value(0, Z_INDEX, 1.0);
        self.tableau.set_value(0, RHS_INDEX, 0.0);

        for (name, variable) in self.model.variables() {
            let column = self.tableau.add_column();
            self.variable_column
                .insert(name.clone(), ColumnType::Single(column));

            self.tableau
                .set_value(0, column, -variable.objective_value());
        }

        self.basic_variable.push(Z_INDEX);
    }

    /// Adds a constraint to the tableau.
    fn create_constraint(&mut self, constraint: &Constraint) {
        let row = self.tableau.add_row();

        match constraint.constraint_type() {
            ConstraintType::LessThan(rhs) => {
                self.tableau.set_value(row, RHS_INDEX, *rhs);
                self.create_slack_variable(row);
            }
            ConstraintType::Equals(_rhs) => {
                todo!();
            }
            ConstraintType::GreaterThan(_rhs) => {
                todo!();
            }
        }

        for (variable_name, coefficient) in constraint.coefficients() {
            let column = self.variable_column.get(variable_name);

            if let None = column {
                log::warn!(
                    "Constraint {} has a coefficient for invalid variable {}",
                    constraint.name(),
                    variable_name
                );
                continue;
            }

            let column = column.unwrap();
            match column {
                ColumnType::Single(index) => {
                    self.tableau.set_value(row, *index, *coefficient);
                }
            }
        }
    }

    fn create_slack_variable(&mut self, row: usize) {
        let column = self.tableau.add_column();
        self.tableau.set_value(row, column, 1.0);
        self.basic_variable.push(column);
    }

    /// If should continue pivotting.
    fn should_continue(&self) -> bool {
        for i in 2..self.tableau.num_cols() {
            if self.get_reduced_cost(i) < 0.0 {
                return true;
            }
        }
        false
    }

    fn pivot(&mut self, pivot_row: usize, pivot_column: usize) {
        log::trace!("Current value = {}", self.get_objective_value());

        self.normalize_pivot_row(pivot_row, pivot_column);
        for current_row in 0..self.tableau.num_rows() {
            if current_row == pivot_row {
                continue;
            }

            let ratio = -self.tableau.get_value(current_row, pivot_column);
            for current_column in 0..self.tableau.num_cols() {
                let new_value = self.tableau.get_value(current_row, current_column)
                    + ratio * self.tableau.get_value(pivot_row, current_column);
                self.tableau
                    .set_value(current_row, current_column, new_value);
            }
        }

        self.basic_variable[pivot_row] = pivot_column;
    }

    /// Makes the coefficient of the entering variable equals 1.
    fn normalize_pivot_row(&mut self, row: usize, column: usize) {
        let ratio = self.tableau.get_value(row, column);

        for i in 0..self.tableau.num_cols() {
            let new_value = self.tableau.get_value(row, i) / ratio;
            self.tableau.set_value(row, i, new_value);
        }
    }

    /// Returns the next variable that should enter the basis.
    fn choose_pivot_column(&self) -> usize {
        let mut next_column: usize = 2;
        let mut best_value = self.get_reduced_cost(2);

        for i in 3..self.tableau.num_cols() {
            let value = self.get_reduced_cost(i);
            if value < best_value {
                next_column = i;
                best_value = value;
            }
        }
        next_column
    }

    /// Chooses the row that should be pivotted (or the variable that will
    /// leave the basis.
    fn choose_pivot_row(&self, entering_variable: usize) -> usize {
        let mut min_ratio = f64::MAX;
        let mut row = 0;

        for i in 1..self.tableau.num_rows() {
            if self.tableau.get_value(i, entering_variable) <= 0.0 {
                continue;
            }

            let ratio =
                self.tableau.get_value(i, RHS_INDEX) / self.tableau.get_value(i, entering_variable);

            if ratio < min_ratio {
                min_ratio = ratio;
                row = i;
            }
        }
        row
    }

    /// Returns the reduced cost of specified column.
    fn get_reduced_cost(&self, column: usize) -> f64 {
        self.tableau.get_value(0, column)
    }

    /// Returns the current objective value.
    fn get_objective_value(&self) -> f64 {
        self.tableau.get_value(0, RHS_INDEX)
    }
}
