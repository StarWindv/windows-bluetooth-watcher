use std::collections::HashMap;
use super::super::types::{
    diff::Diff,
    diff_tool::DiffTool,
    serialize_format::SerializeFormat,
    device::Device
};


use pyo3::{pymethods, PyErr, PyResult};


#[pymethods]
impl DiffTool {
    #[new]
    pub fn new() -> PyResult<Self> {
        Ok(Self {})
    }

    #[staticmethod]
    pub fn diff(old: Vec<Device>, new: Vec<Device>) -> Diff {
        let old_map: HashMap<String, Device> = old
            .into_iter()
            .map(|d| (d.id.clone(), d))
            .collect();

        let new_map: HashMap<String, Device> = new
            .into_iter()
            .map(|d| (d.id.clone(), d))
            .collect();

        let mut connected = Vec::new();
        let mut disconnected = Vec::new();

        for (id, new_dev) in new_map.clone() {
            match old_map.get(&id) {
                Some(old_dev) => {
                    if old_dev.status != new_dev.status {
                        if new_dev.status == "Connected" {
                            connected.push(new_dev);
                        } else if new_dev.status == "Disconnected" {
                            disconnected.push(new_dev);
                        }
                    }
                }

                None => {
                    if new_dev.status == "Connected" {
                        connected.push(new_dev);
                    } else if new_dev.status == "Disconnected" {
                        disconnected.push(new_dev);
                    }
                }
            }
        }
        for (id, old_dev) in old_map {
            if !new_map.contains_key(&id) {
                disconnected.push(old_dev);
            }
        }
        Diff {
            connected,
            disconnected,
        }
    }

    #[staticmethod]
    pub fn to_json_str(devices: Vec<Device>) -> Result<String, PyErr> {
        #![allow(clippy::wrong_self_convention)]
        DiffTool::serialize_to(devices, SerializeFormat::Json)
    }

    #[staticmethod]
    pub fn serialize_to(devices: Vec<Device>, to: SerializeFormat) -> Result<String, PyErr> {
        match to {
            SerializeFormat::Json => {
                Ok(serde_json::to_string_pretty(&devices)
                    .unwrap_or_else(|_| "[]".to_string()))
            }
            SerializeFormat::Yaml => {
                Ok(serde_yaml::to_string(&devices).unwrap_or_else(|_| "[]".to_string()))
            }
        }
    }
}
