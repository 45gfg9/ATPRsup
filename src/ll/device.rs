use rusb::{DeviceHandle, Error, Result};

use crate::ll::ATPR;

use super::ATPRContext;

/// Open device with specified VID, PID, vendor name, product name
fn open_device(
    vid: u16,
    vendor_name: &str,
    pid: u16,
    product_name: &str,
) -> Result<DeviceHandle<ATPRContext>> {
    // reference: avrdude/usbasp.c
    let mut err = Some(Error::NotFound);
    for (desc, device) in rusb::devices()
        .expect("Cannot retrieve device list")
        .iter()
        .map(|dev| (dev.device_descriptor().unwrap(), dev))
        .filter(|(desc, _)| desc.vendor_id() == vid && desc.product_id() == pid)
    {
        let device = match device.open() {
            Ok(handle) => handle,
            Err(e) => {
                err = Some(Error::Access);
                log::warn!("Cannot open USB device: {}", e);
                continue;
            }
        };
        err = None;
        match device.read_manufacturer_string_ascii(&desc) {
            Ok(str) => {
                log::debug!("Found device from vendor \"{}\"", str);
                if str != vendor_name {
                    err = Some(Error::NotFound)
                }
            }
            Err(e) => {
                err = Some(Error::Io);
                log::warn!("Cannot query manufacturer for device: {}", e);
            }
        }
        match device.read_product_string_ascii(&desc) {
            Ok(str) => {
                log::debug!("Found product \"{}\"", str);
                if str != product_name {
                    err = Some(Error::NotFound)
                }
            }
            Err(e) => {
                err = Some(Error::Io);
                log::warn!("Cannot query product for device: {}", e);
            }
        }
        if err.is_none() {
            return Ok(device);
        }
    }
    Err(err.unwrap()) // err must be Some
}

/// Get an ATPR device handle.
#[allow(non_snake_case)]
pub fn getATPR() -> Result<ATPR> {
    // Shared VID & PID provided by Objective Development
    const VID: u16 = 0x16C0;
    const PID: u16 = 0x05DC;
    const VENDOR: &str = "45gfg9.net";
    const PRODUCT: &str = "ATPR";

    open_device(VID, VENDOR, PID, PRODUCT).map(ATPR::new)
}
