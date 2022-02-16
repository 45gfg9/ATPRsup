use crate::ll::ATPR;
use env_logger::{Builder, Target};
use opt::Opt;
use structopt::StructOpt;

mod avr;
mod ll;
mod opt;

fn main() {
    Builder::from_default_env().target(Target::Stdout).init();

    // let opt = Opt::from_args();
    // println!("{:#?}", opt);

    let atpr: ATPR = ll::getATPR().expect("ATPR not found");
    let ver = atpr.version();
    println!("{:?}", ver);
}
