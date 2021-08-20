fn main() {
    for device in rusb::devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();
        let handle = device.open().unwrap();

        println!(
            "Bus {:03} Device {:03}: ID {:04x}:{:04x} {}{}{}",
            device.bus_number(),
            device.address(),
            device_desc.vendor_id(),
            device_desc.product_id(),
            handle
                .read_manufacturer_string_ascii(&device_desc)
                .map_or(String::new(), |s| format!("{} ", s)),
            handle.read_product_string_ascii(&device_desc).unwrap(),
            handle
                .read_serial_number_string_ascii(&device_desc)
                .map_or(String::new(), |s| format!("  Serial: {}", s)),
        );
    }
}
