mod matrix;
pub mod simplex;
mod tableau;

use std::collections::HashMap;

#[macro_export]
macro_rules! variable {
    ($name:literal) => {
        $crate::Variable::new($name)
    };

    ($name:literal >= $lower_bound:literal) => {
        $crate::Variable::new(String::from($name)).with_lower_bound(Some($lower_bound))
    };

    ($lower_bound:literal <= $name:literal <= $upper_bound:literal) => {
        $crate::Variable::new(String::from($name))
            .with_lower_bound(Some($lower_bound))
            .with_upper_bound(Some($upper_bound))
    };
}

#[macro_export]
macro_rules! constraint {
    ($name:literal <= $rhs:literal) => {
        $crate::Constraint::new(String::from($name), ConstraintType::LessThan($rhs))
    };

    ($name:ident <= $rhs:ident) => {
        $crate::Constraint::new($name, ConstraintType::LessThan($rhs))
    };
}

#[derive(Clone)]
pub enum ConstraintType {
    LessThan(f64),
    Equals(f64),
    GreaterThan(f64),
}

#[derive(Clone)]
pub struct Variable {
    name: String,
    lower_bound: Option<f64>,
    upper_bound: Option<f64>,
    objective_value: f64,
}

#[derive(Clone)]
pub struct Constraint {
    name: String,
    coefficients: HashMap<String, f64>,
    constraint_type: ConstraintType,
}

#[derive(Clone)]
struct VariableHandler {
    variables: HashMap<String, Variable>,
}

#[derive(Clone)]
struct ConstraintHandler {
    constraints: HashMap<String, Constraint>,
}

#[derive(Clone)]
pub struct Model {
    variable_handler: VariableHandler,
    constraint_handler: ConstraintHandler,
}

impl Variable {
    pub fn new(name: String) -> Variable {
        Variable {
            name,
            lower_bound: None,
            upper_bound: None,
            objective_value: 0.0,
        }
    }

    pub fn with_objective(mut self, objective_value: f64) -> Self {
        self.objective_value = objective_value;
        self
    }

    pub fn with_lower_bound(mut self, lower_bound: Option<f64>) -> Self {
        self.lower_bound = lower_bound;
        self
    }

    pub fn with_upper_bound(mut self, upper_bound: Option<f64>) -> Self {
        self.upper_bound = upper_bound;
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn objective_value(&self) -> f64 {
        self.objective_value
    }
}

impl Constraint {
    pub fn new(name: String, constraint_type: ConstraintType) -> Constraint {
        Constraint {
            name,
            coefficients: HashMap::new(),
            constraint_type,
        }
    }

    pub fn set_coefficient(&mut self, variable: &Variable, coefficient: f64) {
        let name = variable.name().to_string();
        self.coefficients.insert(name, coefficient);
    }

    fn coefficients(&self) -> &HashMap<String, f64> {
        &self.coefficients
    }

    fn name(&self) -> &str {
        &self.name
    }

    /// Returns the rhs of the constraint.
    fn rhs(&self) -> f64 {
        match self.constraint_type {
            ConstraintType::LessThan(rhs) | ConstraintType::Equals(rhs) | ConstraintType::GreaterThan(rhs) => rhs
        }
    }

    fn constraint_type(&self) -> &ConstraintType {
        &self.constraint_type
    }

    fn get_coefficient(&self, variable: &Variable) -> f64 {
        self.coefficients
            .get(variable.name())
            .map_or(0.0, |c| c.clone())
    }

    /// Fixes the negative RHS by multiplying the constraint by -1.
    fn fix_negative_rhs(&mut self) {
        match self.constraint_type {
            ConstraintType::LessThan(rhs) => {
                assert!(rhs < 0.0);
                self.constraint_type = ConstraintType::GreaterThan(-rhs);
            },
            ConstraintType::Equals(rhs) => {
                assert!(rhs < 0.0);
                self.constraint_type = ConstraintType::Equals(-rhs);
            },
            ConstraintType::GreaterThan(rhs) => {
                assert!(rhs < 0.0);
                self.constraint_type = ConstraintType::LessThan(-rhs);
            },
        }

        for coefficient in &mut self.coefficients.values_mut() {
            *coefficient = -*coefficient;
        }
    }
}

impl Model {
    pub fn new() -> Model {
        Model {
            variable_handler: VariableHandler::new(),
            constraint_handler: ConstraintHandler::new(),
        }
    }

    pub fn variables(&self) -> &HashMap<String, Variable> {
        &self.variable_handler.variables
    }

    pub fn add_variable(&mut self, variable: Variable) {
        self.variable_handler.add(variable);
    }

    pub fn add_constraint(&mut self, constraint: Constraint) {
        self.constraint_handler.add(constraint);
    }
}

impl VariableHandler {
    fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    fn add(&mut self, variable: Variable) {
        self.variables.insert(variable.name().to_string(), variable);
    }
}

impl ConstraintHandler {
    fn new() -> Self {
        Self {
            constraints: HashMap::new(),
        }
    }

    fn add(&mut self, constraint: Constraint) {
        self.constraints
            .insert(constraint.name().to_string(), constraint);
    }

    // TODO: Lower bounds
    // TODO: Fixed values
    // TODO: Negative variables
    fn add_bounds_constraints(&mut self, variable: &Variable) {
        if variable.upper_bound.is_some() {
            self.add_upper_bound_constraint(variable);
        }
    }

    fn add_upper_bound_constraint(&mut self, variable: &Variable) {
        assert!(variable.upper_bound.is_some());
        let ub = variable.upper_bound.unwrap();
        let name = variable.name.to_string() + "_lb";

        let mut constraint = constraint!(name <= ub);
        constraint.set_coefficient(variable, 1.0);
        self.add(constraint);
    }
}
