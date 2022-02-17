use std::rc::Rc;

use crate::ll::ATPR;

mod avr;
mod ll;
mod opt;

fn main() {
    let opt = opt::parse_opt();

    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Trace)
        .init()
        .unwrap();

    println!("{:#?}", opt);

    let atpr: Rc<ATPR> = Rc::new(ll::getATPR().expect("ATPR not found"));
    let ver = atpr.version();
    println!("{:?}", ver);
}
