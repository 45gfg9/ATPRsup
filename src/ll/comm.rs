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

    // libusb wrapper
    pub fn read(&self, request: u8, value: u16, index: u16, buf: &mut [u8]) -> Result<usize> {
        log::trace!(
            "Control In : Request {:#04x}, Value {:#06x}, Index {:#06x}",
            request,
            value,
            index
        );
        log::trace!("==>  {:?}", buf);
        let result = self.handle.read_control(
            Self::REQ_TYPE_CTRL_IN,
            request,
            value,
            index,
            buf,
            Self::TIMEOUT,
        );
        match result {
            Ok(bytes) => log::trace!("{:#5} bytes <== {:?}", bytes, &buf[..bytes - 1]),
            Err(e) => log::trace!("USB Error {}", e),
        }
        result
    }

    pub fn write(&self, request: u8, value: u16, index: u16, buf: &[u8]) -> Result<usize> {
        log::trace!(
            "Control Out: Request {:#04x}, Value {:#06x}, Index {:#06x}",
            request,
            value,
            index,
        );
        log::trace!("==>  {:?}", buf);
        let result = self.handle.write_control(
            Self::REQ_TYPE_CTRL_OUT,
            request,
            value,
            index,
            buf,
            Self::TIMEOUT,
        );
        match result {
            Ok(bytes) => log::trace!("{:#5} bytes <== {:?}", bytes, &buf[..bytes - 1]),
            Err(e) => log::trace!("USB Error {}", e),
        }
        result
    }

    pub fn version(&self) -> Result<Version> {
        // "randomly" generated stamp. Should be echoed back
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
