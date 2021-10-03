use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use strum::{EnumString, EnumVariantNames};

#[derive(EnumString, EnumVariantNames, StructOpt, Serialize, Deserialize, Debug)]
#[strum(ascii_case_insensitive)]
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

#[derive(EnumString, EnumVariantNames, StructOpt, Serialize, Deserialize, Debug)]
#[strum(ascii_case_insensitive)]
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
