#![allow(unused)]

use aderyn_driver::driver;

fn main() {
    use pyo3::prelude::*;

    #[pyfunction]
    fn generate_report(root: String, output: String) {
        let args = driver::Args {
            root,
            output,
            no_snippets: false,       // TODO support this later
            skip_build: false,        // TODO support this later
            skip_cloc: false,         // TODO support this later
            scope: None,              // TODO support this later
            exclude: None,            // TODO support this later
            stdout: false,            // TODO support this later
            skip_update_check: false, // TODO support this later
        };
        driver::drive(args);
    }

    /// A Python module implemented in Rust. The name of this function must match
    /// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
    /// import the module.
    #[pymodule]
    fn aderynpy(_py: Python, m: &PyModule) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(generate_report, m)?)?;

        Ok(())
    }
}
