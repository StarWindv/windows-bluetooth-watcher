use pyo3::pyclass;

#[pyclass(from_py_object, get_all)]
#[derive(Debug, Clone, PartialEq)]
pub enum EventsType {
    Connected,
    Disconnected,
    All,
}