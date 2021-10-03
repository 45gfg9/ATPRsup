use structopt::StructOpt;
use strum::VariantNames;

use crate::avr::{Interface, Memory};

#[derive(Debug, StructOpt)]
#[structopt(name = "atprsup")]
pub struct Opt {
    /// Part name
    #[structopt(short, long)]
    pub part: Option<String>,

    /// Interface
    #[structopt(
        possible_values = Interface::VARIANTS,
        case_insensitive = true
    )]
    pub interface: Interface,

    /// Memory
    #[structopt(
        possible_values = Memory::VARIANTS,
        case_insensitive = true
    )]
    pub memory: Memory,

    /// Verbosity (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,
}
