use pyo3::pyclass;
use serde::Serialize;

#[pyclass(from_py_object, get_all)]
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum EventsType {
    Connected,
    Disconnected,
    All,
}