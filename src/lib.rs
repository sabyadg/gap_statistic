#![feature(proc_macro, specialization)]

pub extern crate pyo3;
pub extern crate ndarray;
pub extern crate num;

use pyo3::prelude::*;
use pyo3::py::modinit as pymodinit;

pub mod gap_statistic;


#[pymodinit(gap_statistic)]
fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "gap_statistic")]
    fn gap_statistic_py(data: Vec<Vec<f64>>, cluster_range: Vec<u32>) -> PyResult<u32> {
       Ok(gap_statistic::optimalK())
    }

    Ok(())
}
