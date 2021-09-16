use structopt::StructOpt;
use strum_macros::{EnumString, EnumVariantNames};

#[derive(EnumString, EnumVariantNames, StructOpt, Debug)]
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
}

#[derive(EnumString, EnumVariantNames, StructOpt, Debug)]
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

pub struct AVR {
    part_name: String,

    interfaces: Vec<Interface>,

    memories: Vec<Memory>,
}
