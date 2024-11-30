use pyo3::prelude::*;

// #[pymodule]
// #[pyo3(name="_lib_name")]
// fn my_lib_name(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
//     m.add_class::<MyPythonRustClass>()?;
//     Ok(())
// }

// struct MyPythonRustClass {
//     name : String
// }

// fn main() {
//     println!("Hello, world!");
// }

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn utils(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}