use rustplex::*;

fn main() {
    env_logger::init();

    let mut model = Model::new();

    let x = variable!("x" >= 0.0).with_objective(3.0);
    let y = variable!("y" >= 0.0).with_objective(2.0);

    let mut c1 = constraint!("c1" <= 18.0);
    c1.set_coefficient(&x, 2.0);
    c1.set_coefficient(&y, 1.0);
    model.add_constraint(c1);

    let mut c2 = constraint!("c2" <= 42.0);
    c2.set_coefficient(&x, 2.0);
    c2.set_coefficient(&y, 3.0);
    model.add_constraint(c2);

    let mut c3 = constraint!("c3" <= 24.0);
    c3.set_coefficient(&x, 3.0);
    c3.set_coefficient(&y, 1.0);
    model.add_constraint(c3);

    model.add_variable(x);
    model.add_variable(y);

    let mut solver = simplex::Solver::new(&model);
    solver.solve();
}
