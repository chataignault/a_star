use ordered_float::OrderedFloat;
use pyo3::marker::Python;
use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::FromPyObject;

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

    // let path = a_star::a_star(start, goal, neighbours, h, d);

    //     let result_node = result_state.node.to_object(py);
    Ok(PyState {
        node: PyList::new(
            py,
            vec![PyState::new(PyList::new(py, vec![1, 2])?.into(), 2.)],
        )?
        .into(),
        cost: 2.,
    })
}

#[derive(Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Debug, FromPyObject)]
struct Point(i32, i32);

#[pyfunction]
fn path_finder_manhattan_2d(
    py_start: PyObject,
    py_goal: PyObject,
    py: Python,
) -> PyResult<PyObject> {
    let start = py_start.extract::<Point>(py)?;
    let goal = py_goal.extract::<Point>(py)?;

    // Manhattan distance heuristic
    let h = |p: &Point| (goal.0 - p.0).abs() as f64 + (goal.1 - p.1).abs() as f64;

    // Get valid neighbors
    let neighbours = |p: &Point| {
        let mut n = Vec::new();
        for &(dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let x = p.0 + dx;
            let y = p.1 + dy;
            if x >= 0 && x <= 2 && y >= 0 && y <= 2 {
                n.push(Point(x, y));
            }
        }
        n
    };

    // Distance between adjacent points is 1.0
    let d = |_: &Point, _: &Point| 1.0;

    match a_star::a_star(start, goal.clone(), neighbours, h, d) {
        Some(path) => Ok(PyList::new(
            py,
            path.into_iter()
                .map(|p| PyList::new(py, vec![p.0, p.1]).unwrap()),
        )?
        .into()),
        None => Ok(PyList::empty(py).into()),
    }
}

#[pymodule]
fn utils(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyState>()?;
    m.add_function(wrap_pyfunction!(path_finder, m)?)?;
    m.add_function(wrap_pyfunction!(path_finder_manhattan_2d, m)?)?;
    Ok(())
}
