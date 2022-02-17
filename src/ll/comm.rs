use std::time::{Duration, SystemTime};

use rusb::{DeviceHandle, Result};

use super::ATPRContext;

#[derive(Debug)]
pub struct Version(u8, u8, u8);

pub struct ATPR {
    handle: DeviceHandle<ATPRContext>,
}

impl ATPR {
    const TIMEOUT: Duration = Duration::from_secs(2);

    // Type = Vendor, Recipient = Device
    const REQ_TYPE_CTRL_IN: u8 = 192;
    const REQ_TYPE_CTRL_OUT: u8 = 64;

    pub fn new(handle: DeviceHandle<ATPRContext>) -> Self {
        Self { handle }
    }

    pub fn get_libusb_handle(&self) -> &DeviceHandle<ATPRContext> {
        &self.handle
    }

    pub fn version(&self) -> Result<Version> {
        let stamp: u8 = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .subsec_millis() as _;

        let mut buf: [u8; 4] = [0; 4];
        self.handle
            .read_control(
                Self::REQ_TYPE_CTRL_IN,
                0xFF,
                stamp as _,
                0,
                &mut buf,
                Self::TIMEOUT,
            )
            .map(|bytes| {
                assert_eq!((4, stamp), (bytes, buf[0]), "Wrong response from firmware");
                Version(buf[1], buf[2], buf[3])
            })
    }
}
