use russell_lab::{format_scientific, get_num_threads, set_num_threads, StrError};
use russell_ode::prelude::*;
use russell_sparse::Genie;
use structopt::StructOpt;

/// Command line options
#[derive(StructOpt)]
#[structopt(name = "BrusselatorPDE", about = "Solve the Brusselator PDE System")]
struct Options {
    /// Second book equations
    #[structopt(long)]
    first_book: bool,

    /// Number of points
    #[structopt(long, default_value = "129")]
    npoint: usize,

    /// Use DoPri5 instead of Radau5
    #[structopt(long)]
    dopri5: bool,

    /// Use DoPri8 instead of Radau5
    #[structopt(long)]
    dopri8: bool,

    /// Negative of the exponent of the tolerance (e.g., 4 yields 1e-4; note that abs_tol = rel_tol)
    #[structopt(long, default_value = "4")]
    neg_exp_tol: i32,

    /// Disable concurrency when solving the real and complex systems
    #[structopt(long)]
    serial: bool,

    /// Number of BLAS threads
    #[structopt(long, default_value = "4")]
    blas_nt: usize,

    /// Use MUMPS solver
    #[structopt(long)]
    mumps: bool,
}

fn main() -> Result<(), StrError> {
    // parse options
    let opt = Options::from_args();
    if opt.npoint < 2 {
        return Err("npoint must be ≥ 2 ");
    }
    if opt.neg_exp_tol < 1 || opt.neg_exp_tol > 30 {
        return Err("neg_exp_tol must satisfy: 1 ≤ net ≤ 30");
    }
    if opt.blas_nt < 1 || opt.blas_nt > 32 {
        return Err("blas_nt must satisfy: 1 ≤ net ≤ 32");
    }

    // set the number of BLAS threads
    set_num_threads(opt.blas_nt);

    // get get ODE system
    let alpha = if opt.first_book { 2e-3 } else { 0.1 };
    let (system, mut data, mut args) = Samples::brusselator_pde(alpha, opt.npoint, !opt.first_book, false);

    // set configuration parameters
    let mut params = if opt.dopri8 {
        Params::new(Method::DoPri8)
    } else if opt.dopri5 {
        Params::new(Method::DoPri5)
    } else {
        Params::new(Method::Radau5)
    };
    let tol = f64::powi(10.0, -opt.neg_exp_tol);
    params.step.h_ini = 1e-6;
    params.radau5.concurrent = !opt.serial;
    params.set_tolerances(tol, tol, None)?;
    params.newton.genie = if opt.mumps { Genie::Mumps } else { Genie::Umfpack };

    // solve the ODE system
    let mut solver = OdeSolver::new(params, &system)?;
    solver.solve(&mut data.y0, data.x0, 0.05, None, None, &mut args)?;

    // print stat
    let stat = solver.stats();
    println!("Second-book problem              = {}", !opt.first_book);
    println!("Number of points along x and y   = {}", opt.npoint);
    println!("Tolerance (abs_tol = rel_tol)    = {}", format_scientific(tol, 8, 2));
    println!("Concurrent real and complex sys  = {}", !opt.serial);
    println!("Problem dimension (ndim)         = {}", system.get_ndim());
    println!("Number of non-zeros (jac_nnz)    = {}", system.get_jac_nnz());
    println!("Number of BLAS threads           = {}", get_num_threads());
    println!("Linear solver                    = {:?}", params.newton.genie);
    println!("{}", stat);
    Ok(())
}
