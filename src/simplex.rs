use crate::tableau::Tableau;
use crate::Model;

pub struct Solver {
    model: Model,
    tableau: Tableau,
}

impl Solver {
    pub fn new(model: &Model) -> Solver {
        log::debug!("Creating a simplex solver");

        let mut preprocessed_model = model.clone();
        Self::preprocess_model(&mut preprocessed_model);

        let solver = Solver {
            tableau: Tableau::new(&preprocessed_model),
            model: preprocessed_model,
        };

        return solver;
    }

    pub fn solve(&mut self) {
        log::info!("Starting to solve simplex model");

        while self.should_continue() {
            let column = self.choose_pivot_column();
            let row = self.choose_pivot_row(column);
            self.pivot(row, column);
        }
        log::info!("Final value = {}", self.tableau.get_objective_value());
    }

    /// If should continue pivotting.
    fn should_continue(&self) -> bool {
        self.tableau.has_negative_reduced_cost()
    }

    fn pivot(&mut self, pivot_row: usize, pivot_column: usize) {
        log::trace!("Current value = {}", self.tableau.get_objective_value());

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

        self.tableau.set_basic_variable(pivot_row, pivot_column);
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
        let mut best_value = self.tableau.get_reduced_cost(2);

        for i in 3..self.tableau.num_cols() {
            let value = self.tableau.get_reduced_cost(i);
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
            if let Some(ratio) = self.get_ratio_test(i, entering_variable) {
                if ratio < min_ratio {
                    min_ratio = ratio;
                    row = i;
                }
            }
        }
        row
    }

    fn get_ratio_test(&self, row: usize, pivot_column: usize) -> Option<f64> {
        if self.tableau.get_value(row, pivot_column) <= 0.0 {
            return None;
        }

        let ratio = self.tableau.get_rhs(row) / self.tableau.get_value(row, pivot_column);
        Some(ratio)
    }

    /// Preprocess the model, adding needed constraints.

    /// This function currently:
    /// 1. Fixes constraints that have negative RHS by multiplying everything by -1;
    /// 2. Creates constraints for the variable bounds (and fixed variables).
    fn preprocess_model(model: &mut Model) {
        for constraint in model.constraint_handler.constraints.values_mut() {
            if constraint.rhs() < 0.0 {
                constraint.fix_negative_rhs();
            }
        }

        for variable in model.variable_handler.variables.values() {
            model.constraint_handler.add_bounds_constraints(variable);
        }
    }
}
