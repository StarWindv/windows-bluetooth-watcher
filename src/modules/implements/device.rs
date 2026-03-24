use super::super::types::{
    device::Device,
    diff_tool::DiffTool,
    serialize_format::SerializeFormat
};


impl Device {
    pub fn __str__(&self) -> String {
        DiffTool::serialize_one(&self.clone(), SerializeFormat::Json).unwrap()
    }

    pub fn __repr__(&self) -> String {
        self.__str__()
    }
}
