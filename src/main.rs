use crate::ll::ATPR;

mod avr;
mod ll;
mod opt;

fn main() {
    env_logger::init();
    // let opt = Opt::from_args();
    // println!("{:#?}", opt);

    let atpr: ATPR = ll::getATPR().expect("ATPR not found");
    let ver = atpr.version();
    println!("{:?}", ver);
}
