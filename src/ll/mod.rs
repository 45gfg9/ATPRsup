mod device;

mod comm;

type ATPRContext = rusb::GlobalContext;

pub use comm::ATPR;
pub use device::getATPR;
