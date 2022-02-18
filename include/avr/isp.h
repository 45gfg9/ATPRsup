/* ATPRsup - Software for ATPR
 * Copyright (C) 2021-2022 45gfg9
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

#ifndef __ATPRSUP_ISP_H__
#define __ATPRSUP_ISP_H__

#include "avr/avr.h"
#include "ll.h"

#include <libusb-1.0/libusb.h>

namespace AVR {
    class ISP : public Program {
        uint8_t ispSpeed;
        uint8_t sigBuf[3];

    public:
        explicit ISP(ATPR *handle, uint8_t ispSpeed = 4);

        ~ISP() override;

        bool begin() override;

        bool chipErase() override;

        bool isReady() override;

        bool latchData() override;

        uint8_t readFlash(uint32_t address) override;

        bool writeFlash(uint32_t address, uint8_t data) override;

        bool loadFlashPage(uint32_t address, const uint8_t *data, uint16_t pageSize) override;

        bool flushFlashPage() override;

        uint8_t readEeprom(uint32_t address) override;

        bool writeEeprom(uint32_t address, uint8_t data) override;

        uint8_t readFuse(uint8_t address) override;

        bool writeFuse(uint8_t address, uint8_t data) override;

        uint8_t readLockBits() override;

        const uint8_t *readSignature() override;
    };
}
#endif // __ATPRSUP_ISP_H__
