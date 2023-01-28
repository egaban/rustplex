mod matrix;
pub mod simplex;

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
}

pub enum ConstraintType {
    LessThan(f64),
    Equals(f64),
    GreaterThan(f64),
}

pub struct Variable {
    name: String,
    lower_bound: Option<f64>,
    upper_bound: Option<f64>,
    objective_value: f64,
}

pub struct Constraint {
    name: String,
    coefficients: HashMap<String, f64>,
    constraint_type: ConstraintType,
}

pub struct Model {
    variables: HashMap<String, Variable>,
    constraints: HashMap<String, Constraint>,
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

    fn constraint_type(&self) -> &ConstraintType {
        &self.constraint_type
    }
}

impl Model {
    pub fn new() -> Model {
        Model {
            variables: HashMap::new(),
            constraints: HashMap::new(),
        }
    }

    pub fn add_variable(&mut self, variable: Variable) {
        log::trace!("Adding variable {}", variable.name());
        let name = variable.name.clone();
        self.variables.insert(name, variable);
    }

    pub fn add_constraint(&mut self, constraint: Constraint) {
        log::trace!("Adding constraint {}", constraint.name());
        let name = constraint.name.clone();
        self.constraints.insert(name, constraint);
    }

    pub fn variables(&self) -> &HashMap<String, Variable> {
        &self.variables
    }
}
