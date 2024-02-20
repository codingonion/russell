use russell_lab::{approx_eq, Vector};
use russell_ode::{Method, OdeSolver, Params, Samples};

#[test]
fn test_fweuler_hairer_wanner_eq1() {
    let (system, mut data, mut args) = Samples::hairer_wanner_eq1();
    let ndim = system.get_ndim();
    let params = Params::new(Method::FwEuler);
    let mut solver = OdeSolver::new(params, system).unwrap();
    solver
        .solve(&mut data.y0, data.x0, data.x1, data.h_equal, None, &mut args)
        .unwrap();
    let mut analytical = data.y_analytical.unwrap();
    let mut y1_correct = Vector::new(ndim);
    analytical(&mut y1_correct, data.x1);
    approx_eq(data.y0[0], 0.08589790706616637, 1e-15);
    approx_eq(data.y0[0], y1_correct[0], 0.004753);
    let b = solver.bench();
    println!("{}", b);
}
