use crate::ll::ATPR;

mod avr;
mod ll;
mod opt;

fn main() {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Trace)
        .init()
        .unwrap();

    let atpr: ATPR = ll::getATPR().expect("ATPR not found");
    let ver = atpr.version();
    println!("{:?}", ver);
}
