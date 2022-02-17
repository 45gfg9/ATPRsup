use serde::{Deserialize, Serialize};

pub trait Program {
    fn connect();
    fn disconnect();

    fn begin();
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Interface {
    ISP,
    JTAG,
    /// High-Voltage Serial Programming
    HVSP,
    /// High-Voltage Parallel Programming
    HVPP,
    /// debugWire
    DW,
    /// ATxmega PDI
    PDI,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Memory {
    Flash,
    EEPROM,
    /// High Fuse
    HFuse,
    /// Low Fuse
    LFuse,
    /// Extended Fuse
    EFuse,
    /// Lock bits
    Lock,
}

pub trait Program {
    fn connect(&self);
    fn begin(&self) -> bool;
    fn close(self);

    fn chip_erase(&self) -> bool;
    fn is_ready(&self) -> bool;
    fn latch_data(&self) -> bool;

    fn read_flash(&self, address: usize) -> u8;
    fn write_flash(&self, address: usize, data: u8) -> bool;

    fn load_flash_page(&self, address: usize, data: &[u8]);
    fn flush_flash_page(&self) -> bool;

    fn read_eeprom(&self, address: usize) -> u8;
    fn write_eeprom(&self, address: usize, data: u8) -> bool;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AVR {
    pub part_name: String,
}
