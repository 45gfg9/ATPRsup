use rusb::{Device, GlobalContext, Result};

pub fn lsusb() {
    fn ls(device: Device<GlobalContext>) -> Result<()> {
        let device_desc = device.device_descriptor()?;

        let handle = device.open()?;

        log::debug!(
            "Bus {:03} Device {:03}: ID {:04x}:{:04x} {}{}{}",
            device.bus_number(),
            device.address(),
            device_desc.vendor_id(),
            device_desc.product_id(),
            handle
                .read_manufacturer_string_ascii(&device_desc)
                .map_or(String::new(), |s| format!("{} ", s)),
            handle.read_product_string_ascii(&device_desc)?,
            handle
                .read_serial_number_string_ascii(&device_desc)
                .map_or(String::new(), |s| format!("  Serial: {}", s)),
        );

        Ok(())
    }

    for device in rusb::devices().expect("Cannot retrieve device list").iter() {
        match ls(device) {
            Ok(_) => (),
            Err(error) => log::error!("Error: {}", error),
        };
    }
}
