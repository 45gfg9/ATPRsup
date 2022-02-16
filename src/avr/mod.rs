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

#[derive(Serialize, Deserialize, Debug)]
pub struct AVR {
    pub part_name: String,

    pub interfaces: Vec<Interface>,

    pub memories: Vec<Memory>,
}
