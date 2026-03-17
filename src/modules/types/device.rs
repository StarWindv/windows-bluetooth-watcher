use pyo3::pyclass;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[pyclass(from_py_object, get_all)]
pub struct Device {
    pub name: String,
    pub id  : String,
    pub addr: u64,
    pub status: String,
}
