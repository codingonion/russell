use plotpy::{Curve, Plot};
use russell_lab::StrError;
use russell_ode::prelude::*;
use serde::Deserialize;
use std::{env, fs::File, io::BufReader, path::Path};

// This example solves the brusselator equation
//
// This example corresponds to Fig 16.4 on page 116 of the reference.
// See also Eq (16.12) on page 116 of the reference.
//
// # Reference
//
// * Hairer E, Nørsett, SP, Wanner G (2008) Solving Ordinary Differential Equations I.
//   Non-stiff Problems. Second Revised Edition. Corrected 3rd printing 2008. Springer Series
//   in Computational Mathematics, 528p

fn main() -> Result<(), StrError> {
    // get the ODE system
    let (system, mut data, mut args, y_ref) = Samples::brusselator_ode();

    // solver
    let params = Params::new(Method::DoPri8);
    let mut solver = OdeSolver::new(params, &system)?;

    // enable dense output
    let mut out = Output::new();
    let h_out = 0.01;
    let selected_y_components = &[0, 1];
    out.enable_dense(h_out, selected_y_components)?;

    // solve the problem
    solver.solve(&mut data.y0, data.x0, data.x1, None, Some(&mut out), &mut args)?;

    // print the results and stats
    println!("y_russell     = {:?}", data.y0.as_data());
    println!("y_mathematica = {:?}", y_ref.as_data());
    println!("{}", solver.bench());

    // load reference data (from Mathematica)
    let math = ReferenceData::read("data/reference/brusselator_mathematica.json")?;

    // plot the results
    let mut curve1 = Curve::new();
    let mut curve2 = Curve::new();
    curve1.set_label("russell");
    curve2.set_label("mathematica");
    curve1.draw(out.dense_y.get(&0).unwrap(), out.dense_y.get(&1).unwrap());
    curve2.set_marker_style(".").set_line_style("None");
    curve2.draw(&math.y0, &math.y1);

    // save figure
    let mut plot = Plot::new();
    plot.add(&curve1)
        .add(&curve2)
        .legend()
        .set_title("Brusselator ODE - DoPri8")
        .set_equal_axes(true)
        .set_figure_size_points(600.0, 600.0)
        .grid_and_labels("$y_0$", "$y_1$")
        .save("/tmp/russell_ode/brusselator_dopri8.svg")
}

#[derive(Deserialize)]
struct ReferenceData {
    #[allow(unused)]
    pub x: Vec<f64>,
    pub y0: Vec<f64>,
    pub y1: Vec<f64>,
}

impl ReferenceData {
    pub fn read(rel_path: &str) -> Result<Self, StrError> {
        let full_path = format!("{}/{}", env::var("CARGO_MANIFEST_DIR").unwrap(), rel_path);
        let path = Path::new(full_path.as_str()).to_path_buf();
        let input = File::open(path).map_err(|_| "cannot open file")?;
        let buffered = BufReader::new(input);
        let data = serde_json::from_reader(buffered).map_err(|_| "cannot parse JSON file")?;
        Ok(data)
    }
}