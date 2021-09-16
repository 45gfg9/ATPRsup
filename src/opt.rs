use structopt::StructOpt;
use strum::VariantNames;

use crate::avr::{Interface, Memory};

#[derive(Debug, StructOpt)]
#[structopt(name = "atprsup")]
pub struct Opt {
    /// Part name
    #[structopt(short, long)]
    part: Option<String>,

    /// Interface
    #[structopt(
        possible_values = Interface::VARIANTS,
        case_insensitive = true
    )]
    interface: Interface,

    /// Memory
    #[structopt(
        possible_values = Memory::VARIANTS,
        case_insensitive = true
    )]
    memory: Memory,

    /// Verbosity (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,
}
