use crate::modules::types::{
    diff::Diff,
    diff_tool::DiffTool,
    device::Device
};
use pyo3::{PyResult, pymethods};

#[pymethods]
impl Diff {
    #[new]
    pub fn __init__(connected: Vec<Device>, disconnected: Vec<Device>) -> PyResult<Self> {
        Ok(Self { connected, disconnected })
    }

    pub fn __str__(&self) -> String {
        let connected_json = DiffTool::to_json_str(&*self.connected);
        let disconnected_json = DiffTool::to_json_str(&*self.disconnected);

        format!(
            "{{\n  \"connected\": {:?},\n  \"disconnected\": {:?}\n}}",
            connected_json, disconnected_json
        )
    }
}
