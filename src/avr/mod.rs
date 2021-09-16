use structopt::StructOpt;
use strum_macros::{EnumString, EnumVariantNames};

#[derive(EnumString, EnumVariantNames, StructOpt, Debug)]
#[strum(ascii_case_insensitive)]
pub enum Interface {
    ISP,
    JTAG,
    HVSP,
    PP,
    DW,
}

#[derive(EnumString, EnumVariantNames, StructOpt, Debug)]
#[strum(ascii_case_insensitive)]
pub enum Memory {
    Flash,
    EEPROM,
    HFuse,
    LFuse,
    EFuse,
    Lock,
}

pub struct AVR {
    part_name: String,

    interfaces: Vec<Interface>,

    memories: Vec<Memory>,
}
