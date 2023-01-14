use rustplex::*;

fn main() {
    env_logger::init();

    let mut model = Model::new();

    let x = Variable::new(String::from("x"), 3.0).with_lower_bound(Some(0.0));
    let y = Variable::new(String::from("y"), 2.0).with_lower_bound(Some(0.0));

    let mut c1 = Constraint::new(String::from("c1"), ConstraintType::LessThan(18.0));
    c1.set_coefficient(&x, 2.0);
    c1.set_coefficient(&y, 1.0);
    model.add_constraint(c1);

    let mut c2 = Constraint::new(String::from("c2"), ConstraintType::LessThan(42.0));
    c2.set_coefficient(&x, 2.0);
    c2.set_coefficient(&y, 3.0);
    model.add_constraint(c2);

    let mut c3 = Constraint::new(String::from("c3"), ConstraintType::LessThan(24.0));
    c3.set_coefficient(&x, 3.0);
    c3.set_coefficient(&y, 1.0);
    model.add_constraint(c3);

    model.add_variable(x);
    model.add_variable(y);

    let mut solver = simplex::Solver::new(&model);
    solver.solve();
}
