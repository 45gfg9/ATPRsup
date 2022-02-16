use std::time::Duration;

use rusb::{request_type, DeviceHandle, Result};

use crate::avr::{Interface, Memory};

use super::ATPRContext;

impl Memory {
    fn as_mask(&self) -> u8 {
        match self {
            Memory::Flash => 0x0,
            Memory::EEPROM => 0x1,
            Memory::HFuse => 0x2,
            Memory::LFuse => 0x3,
            Memory::EFuse => 0x4,
            Memory::Lock => 0x5,
        }
    }
}

impl Interface {
    fn as_mask(&self) -> u8 {
        match self {
            Interface::ISP => 0x00,
            Interface::JTAG => 0x10,
            Interface::HVSP => 0x20,
            Interface::HVPP => 0x30,
            Interface::DW => 0x40,
            Interface::PDI => 0x50,
        }
    }
}

pub struct ATPR {
    handle: DeviceHandle<ATPRContext>,
}

impl ATPR {
    const TIMEOUT: Duration = Duration::from_secs(2);

    pub fn new(handle: DeviceHandle<ATPRContext>) -> Self {
        Self { handle }
    }

    fn pack(mem: Memory, int: Interface) -> (u8, u16, u16) {
        let request: u8 = mem.as_mask() | int.as_mask();

        let value: u16 = 0;
        let index: u16 = 0;

        (request, value, index)
    }

    pub fn get_libusb_handle(&self) -> &DeviceHandle<ATPRContext> {
        &self.handle
    }

    pub fn write(&self, mem: Memory, interface: Interface, data: &[u8]) -> Result<usize> {
        let packed = ATPR::pack(mem, interface);
        self.handle.write_control(
            request_type(
                rusb::Direction::Out,
                rusb::RequestType::Vendor,
                rusb::Recipient::Device,
            ),
            packed.0,
            packed.1,
            packed.2,
            data,
            ATPR::TIMEOUT,
        )
    }
}
