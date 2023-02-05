use crate::matrix::Matrix;
use crate::{Constraint, ConstraintType, Model, Variable};
use std::collections::HashMap;

const RHS_INDEX: usize = 0;
const Z_INDEX: usize = 1;
const BIG_M: f64 = 1e150;

enum ColumnType {
    Single(usize),
}

pub struct Tableau {
    matrix: Matrix,
    variable_column: HashMap<String, ColumnType>,
    basic_variable: Vec<usize>,
}

impl Tableau {
    pub fn new(model: &Model) -> Tableau {
        let mut tableau = Tableau {
            matrix: Matrix::new(),
            variable_column: HashMap::new(),
            basic_variable: Vec::new(),
        };

        tableau.create_row0(model);
        tableau.create_constraints(model);
        tableau
    }

    pub fn set_value(&mut self, row: usize, column: usize, value: f64) {
        self.matrix.set_value(row, column, value);
    }

    pub fn get_value(&self, row: usize, column: usize) -> f64 {
        self.matrix.get_value(row, column)
    }

    /// Creates the first row of the tableau, containing the reduced costs.
    fn create_row0(&mut self, model: &Model) {
        self.matrix.add_row();
        self.matrix.add_column();
        self.matrix.add_column();

        self.matrix.set_value(0, Z_INDEX, 1.0);
        self.matrix.set_value(0, RHS_INDEX, 0.0);

        for (name, variable) in model.variables() {
            let column = self.matrix.add_column();
            self.variable_column
                .insert(name.clone(), ColumnType::Single(column));

            self.matrix
                .set_value(0, column, -variable.objective_value());
        }

        self.basic_variable.push(Z_INDEX);
    }

    fn create_constraints(&mut self, model: &Model) {
        for constraint in model.constraint_handler.constraints.values() {
            self.create_constraint(constraint);
        }
    }

    /// Adds a constraint to the tableau.
    fn create_constraint(&mut self, constraint: &Constraint) {
        let row = self.matrix.add_row();

        match constraint.constraint_type() {
            ConstraintType::LessThan(rhs) => {
                self.matrix.set_value(row, RHS_INDEX, *rhs);
                self.create_slack_variable(row);
            }
            ConstraintType::Equals(rhs) => {
                self.matrix.set_value(row, RHS_INDEX, *rhs);
                self.create_artificial_variable(row);
            }
            ConstraintType::GreaterThan(rhs) => {
                self.matrix.set_value(row, RHS_INDEX, *rhs);
                self.create_artificial_variable(row);
                self.create_surplus_variable(row);
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
                    self.matrix.set_value(row, *index, *coefficient);
                }
            }
        }
    }

    fn create_slack_variable(&mut self, row: usize) {
        let column = self.matrix.add_column();
        self.matrix.set_value(row, column, 1.0);
        self.basic_variable.push(column);
    }

    /// Makes an artificial variable for the big M method.
    fn create_artificial_variable(&mut self, row: usize) {
        let column = self.matrix.add_column();
        self.matrix.set_value(row, column, 1.0);
        self.matrix.set_value(0, column, BIG_M);

        // We must make the variable disappear from the row 0, so we can start
        // with the cannonical form.

        for column in 0..self.matrix.num_cols() {
            let new_value = self.matrix.get_value(0, column) - BIG_M * self.matrix.get_value(row, column);
            self.matrix.set_value(0, column, new_value);
        }

        self.basic_variable.push(column);
    }

    fn create_surplus_variable(&mut self, row: usize) {
        let column = self.matrix.add_column();
        self.matrix.set_value(row, column, -1.0);
    }

    /// Returns the reduced cost of specified column.
    pub fn get_reduced_cost(&self, column: usize) -> f64 {
        self.matrix.get_value(0, column)
    }

    pub fn get_rhs(&self, row: usize) -> f64 {
        self.matrix.get_value(row, RHS_INDEX)
    }

    pub fn num_rows(&self) -> usize {
        self.matrix.num_rows()
    }

    pub fn num_cols(&self) -> usize {
        self.matrix.num_cols()
    }

    /// Returns the current objective value.
    pub fn get_objective_value(&self) -> f64 {
        self.matrix.get_value(0, RHS_INDEX)
    }

    /// Returns if any variable still has a negative reduced cost.
    pub fn has_negative_reduced_cost(&self) -> bool {
        for i in 2..self.matrix.num_cols() {
            if self.get_reduced_cost(i) < 0.0 {
                return true;
            }
        }
        false
    }

    pub fn set_basic_variable(&mut self, row: usize, column: usize) {
        self.basic_variable[row] = column;
    }
}
