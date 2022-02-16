use std::time::{Duration, SystemTime};

use crate::ll::ATPR;
use env_logger::{Builder, Target};
use opt::Opt;
use rusb::{request_type, RequestType};
use structopt::StructOpt;

mod avr;
mod ll;
mod opt;

fn main() {
    Builder::from_default_env().target(Target::Stdout).init();

    // let opt = Opt::from_args();
    // println!("{:#?}", opt);

    let atpr: ATPR = ll::getATPR().expect("ATPR not found");

    let stamp: u8 = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as _;

    let mut buf: [u8; 4] = [0; 4];
    let handle = atpr.get_libusb_handle();
    let result = handle.read_control(
        request_type(
            rusb::Direction::In,
            RequestType::Vendor,
            rusb::Recipient::Device,
        ),
        0xff,
        stamp as _,
        0,
        &mut buf,
        Duration::from_secs(1),
    );
    println!("{} {:?} {:?}", stamp, result, buf);
}
