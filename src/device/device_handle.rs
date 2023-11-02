use std::time::Duration;

use crate::error::{Result, RtlsdrError};
use crate::error::RtlsdrError::RtlsdrErr;
use rusb::{Context, UsbContext};

use std::os::unix::io::AsRawFd;
use std::fs::File;


use super::KNOWN_DEVICES;

impl From<std::io::Error> for RtlsdrError {
    fn from(error: std::io::Error) -> Self {
        RtlsdrError::RtlsdrErr(error.to_string())
    }
}
#[derive(Debug)]
pub struct DeviceHandle {
    handle: rusb::DeviceHandle<Context>,
}
impl DeviceHandle {
    
    pub fn open_with_path(devpath: String) -> Result<Self>{
        let mut context = Context::new()?;
        // Open a file descriptor.
        let file = File::open(devpath)?;
        let fd = file.as_raw_fd();

        /*
        Open the device using the file descriptor.
        */
        let handle  = unsafe { context.open_device_with_fd(fd)?};
        Ok(DeviceHandle { handle: handle })

    }
    
    pub fn open(index: usize) -> Result<Self> {
        let mut context = Context::new()?;
        let handle = DeviceHandle::open_device(&mut context, index)?;
        Ok(DeviceHandle { handle: handle })
    }

    pub fn open_device<T: UsbContext>(
        context: &mut T,
        _index: usize,
    ) -> Result<rusb::DeviceHandle<T>> {
        let devices = context.devices().map(|d| d)?;

        let _device = for found in devices.iter() {
            let device_desc = found.device_descriptor().map(|d| d)?;
            for dev in KNOWN_DEVICES.iter() {
                if device_desc.vendor_id() == dev.vid && device_desc.product_id() == dev.pid {
                    return Ok(found.open()?);
                }
            }
        };
        Err(RtlsdrErr(format!("No device found")))
    }

    pub fn claim_interface(&mut self, iface: u8) -> Result<()> {
        Ok(self.handle.claim_interface(iface)?)
    }
    pub fn reset(&mut self) -> Result<()> {
        Ok(self.handle.reset()?)
    }

    pub fn read_control(
        &self,
        request_type: u8,
        request: u8,
        value: u16,
        index: u16,
        buf: &mut [u8],
        timeout: Duration,
    ) -> Result<usize> {
        Ok(self
            .handle
            .read_control(request_type, request, value, index, buf, timeout)?)
    }

    pub fn write_control(
        &self,
        request_type: u8,
        request: u8,
        value: u16,
        index: u16,
        buf: &[u8],
        timeout: Duration,
    ) -> Result<usize> {
        Ok(self
            .handle
            .write_control(request_type, request, value, index, buf, timeout)?)
    }

    pub fn read_bulk(&self, endpoint: u8, buf: &mut [u8], timeout: Duration) -> Result<usize> {
        Ok(self.handle.read_bulk(endpoint, buf, timeout)?)
    }

    
}
