use ordered_float::OrderedFloat;
use pyo3::marker::Python;
use pyo3::prelude::*;
use pyo3::types::PyList;

mod a_star;
use a_star::State;

#[pyclass]
struct PyState {
    #[pyo3(get)]
    pub node: PyObject,
    #[pyo3(get)]
    pub cost: f64,
}

#[pymethods]
impl PyState {
    #[new]
    fn new(node: PyObject, cost: f64) -> Self {
        PyState { node, cost }
    }
}

#[pyfunction]
fn path_finder(
    py_start: &PyState,
    py_goal: &PyState,
    py: Python, // py_neighbors:PyObject,
                // py_h:PyObject,
                // py_d: PyObject
) -> PyResult<PyState> {
    let start = State {
        cost: OrderedFloat(py_start.cost),
        node: py_start.node.extract::<Vec<i32>>(py)?,
    };
    let goal = State {
        cost: OrderedFloat(py_goal.cost),
        node: py_goal.node.extract::<Vec<i32>>(py)?,
    };

    //     let path = a_star::a_star(start, goal, neighbours, h, d);

    //     let result_node = result_state.node.to_object(py);
    Ok(PyState {
        node: PyList::new(py, vec![2, 3, 4])?.into(),
        cost: 2.,
    })
}

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pymodule]
fn utils(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyState>()?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(path_finder, m)?)?;
    Ok(())
}
