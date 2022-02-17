use std::rc::Rc;

use rusb::Result;

use crate::ll::ATPR;

use super::Program;

pub struct ISP {
    handle: Rc<ATPR>,
}

impl From<Rc<ATPR>> for ISP {
    fn from(handle: Rc<ATPR>) -> Self {
        Self { handle }
    }
}

impl Program for ISP {
    fn connect(&self) -> Result<()> {
        log::debug!("Connecting to device");
        self.handle.write(0x00, 0x00, 0x00, &[]).map(|_| ())
    }

    fn begin(&self, isp_speed: u8) -> Result<bool> {
        assert!(isp_speed < 14);
        log::debug!("Begin ISP transfer with speed grade {}", isp_speed);

        let isp_speed = match isp_speed {
            0 => 0b1000, // f_osc / 4
            1 => 0b1101, // f_osc / 8
            2 => 0b1001, // f_osc / 16
            3 => 0b1110, // f_osc / 32
            4 => 0b1010, // f_osc / 64
            5 => 0b1011, // f_osc / 128
            _ => isp_speed - 6,
        };

        let mut buf = [0u8];
        self.handle
            .read(0x00, isp_speed as _, 0x00, &mut buf)
            .map(|_| buf[0] == 0x01)
            .map_err(|e| {
                log::error!("Can't begin: target doesn't answer.");
                e
            })
    }

    fn end(self) -> Result<()> {
        todo!()
    }

    fn chip_erase(&self) -> Result<bool> {
        todo!()
    }

    fn is_ready(&self) -> Result<bool> {
        todo!()
    }

    fn latch_data(&self) -> Result<bool> {
        todo!()
    }

    fn read_flash(&self, address: usize) -> Result<u8> {
        todo!()
    }

    fn write_flash(&self, address: usize, data: u8) -> Result<bool> {
        todo!()
    }

    fn load_flash_page(&self, address: usize, data: &[u8]) {
        todo!()
    }

    fn flush_flash_page(&self) -> Result<bool> {
        todo!()
    }

    fn read_eeprom(&self, address: usize) -> Result<u8> {
        todo!()
    }

    fn write_eeprom(&self, address: usize, data: u8) -> Result<bool> {
        todo!()
    }
}
