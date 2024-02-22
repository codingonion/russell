use russell_lab::format_fortran;
use russell_ode::{Method, OdeSolver, Params, Samples};

#[test]
fn test_dopri5_arenstorf_debug() {
    // get ODE system
    let (system, mut data, mut args) = Samples::arenstorf();

    // set configuration parameters
    let mut params = Params::new(Method::DoPri5);
    params.h_ini = 1e-4;
    params.set_tolerances(1e-7, 1e-7).unwrap();
    params.erk.logging = true;

    // solve the ODE system
    let mut solver = OdeSolver::new(params, system).unwrap();
    solver
        .solve(&mut data.y0, data.x0, data.x1, None, None, &mut args)
        .unwrap();

    // get statistics
    let stat = solver.bench();

    // print and check statistics
    println!("{}", stat);
    println!("y ={}{}", format_fortran(data.y0[0]), format_fortran(data.y0[1]));
    println!("h ={}", format_fortran(stat.h_optimal));
    // assert_eq!(stat.n_function, 1429);
    // assert_eq!(stat.n_jacobian, 0);
    // assert_eq!(stat.n_steps, 238);
    // assert_eq!(stat.n_accepted, 217);
    // assert_eq!(stat.n_rejected, 21);
    // assert_eq!(stat.n_iterations, 0);
    // assert_eq!(stat.n_iterations_max, 0);
}
