use pyo3::prelude::*;

mod a_star;
use a_star::State;

#[pyclass]
struct PyState {
    #[pyo3(get)]
    node: PyObject,
    #[pyo3(get)]
    cost: f64,
}

// #[pyfunction]
// fn path_finder(py_start: &PyState, py_goal: &PyState, py_neighbors:PyObject, py_h:PyObject, py_d: PyObject) -> PyResult<PyState> {
//     let start = State {
//         cost: OrderedFloat(py_start.cost),
//         node: py_start.node.extract(py)?,
//     };
//     let goal = State {
//         cost: OrderedFloat(py_state.cost),
//         node: py_goal.node.extract(py)?,
//     };

//     let path = a_star::a_star(start, goal, neighbours, h, d);

//     let result_node = result_state.node.to_object(py);
//     Ok(PyState {
//         node: result_node,
//         cost: result_state.cost.into_inner(),
//     })}

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pymodule]
fn utils(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyState>()?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    // m.add_function(wrap_pyfunction!(path_finder, m)?)?;
    Ok(())
}
