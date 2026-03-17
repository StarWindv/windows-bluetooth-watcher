use pyo3::pyclass;


#[derive(Debug, Clone)]
#[pyclass(from_py_object)]
pub struct Listener {}
