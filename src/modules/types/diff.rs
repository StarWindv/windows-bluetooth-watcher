use pyo3::pyclass;
use super::super::types::device::Device;


#[pyclass(from_py_object, get_all)]
#[derive(Debug, Clone)]
pub struct Diff {
    pub connected: Vec<Device>,
    pub disconnected: Vec<Device>,
}
