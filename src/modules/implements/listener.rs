use super::super::{
    types::listener::Listener,
    types::device::Device,
    error::ConvertToPyErr,
};

use pyo3::{PyResult, pymethods};

use windows::Devices::{
    Enumeration::DeviceInformation,
    Bluetooth::BluetoothDevice,
    Bluetooth::BluetoothConnectionStatus
};


#[pymethods]
impl Listener {
    #[new]
    pub fn new() -> PyResult<Self> { Ok(Self { }) }

    pub async fn get_all(&self) -> PyResult<Vec<Device>> {
        let mut devices = Vec::new();

        let selector = BluetoothDevice::GetDeviceSelector().auto()?;
        let information = DeviceInformation::FindAllAsyncAqsFilter(&selector)
            .auto()?
            .await
            .auto()?;

        let mut info_vec = Vec::new();
        let size = information.Size().auto()?;
        for i in 0..size {
            let dev_info = information.GetAt(i).auto()?;
            info_vec.push(dev_info);
        }

        for d in info_vec {
            let id = d.Id().auto()?;
            let name = d.Name().auto()?;

            if let Ok(bt) = BluetoothDevice::FromIdAsync(&id).auto()?.await {
                let status = bt.ConnectionStatus().auto()?;
                let addr = bt.BluetoothAddress().auto()?;
                let status_string = match status {
                    BluetoothConnectionStatus::Connected => "Connected",
                    _ => "Disconnected",
                };

                devices.push(Device {
                    name: name.to_string(),
                    id: id.to_string(),
                    addr,
                    status: status_string.to_string(),
                });
            }
        }

        Ok(devices)
    }
}
