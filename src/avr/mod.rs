use rusb::Result;
use serde::{Deserialize, Serialize};


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
    fn connect(&self) -> Result<()>;
    fn begin(&self) -> Result<bool>;
    fn close(self) -> Result<()>;

    fn chip_erase(&self) -> Result<bool>;
    fn is_ready(&self) -> Result<bool>;
    fn latch_data(&self) -> Result<bool>;

    fn read_flash(&self, address: usize) -> Result<u8>;
    fn write_flash(&self, address: usize, data: u8) -> Result<bool>;

    fn load_flash_page(&self, address: usize, data: &[u8]);
    fn flush_flash_page(&self) -> Result<bool>;

    fn read_eeprom(&self, address: usize) -> Result<u8>;
    fn write_eeprom(&self, address: usize, data: u8) -> Result<bool>;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AVR {
    pub part_name: String,
}
