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

#ifndef __ATPRSUP_AVR_H__
#define __ATPRSUP_AVR_H__

#include "ll.h"

#include <cstdint>

namespace AVR {
    enum class Protocol {
        ISP,
        JTAG,
        // High-Voltage Serial Programming
        HVSP,
        // High-Voltage Parallel Programming
        HVPP,
        // debugWire
        DW,
        // ATxmega PDI
        PDI,
    };

    class Program {
    protected:
        ATPR *handle;

    public:
        explicit Program(ATPR *handle);

        virtual ~Program();

        virtual bool begin() = 0;

        virtual bool chipErase() = 0;

        virtual bool isReady() = 0;

        virtual bool latchData() = 0;

        virtual uint8_t readFlash(uint32_t address) = 0;

        virtual bool writeFlash(uint32_t address, uint8_t data) = 0;

        virtual bool loadFlashPage(uint32_t address, const uint8_t *data, uint16_t pageSize) = 0;

        virtual bool flushFlashPage() = 0;

        virtual uint8_t readEeprom(uint32_t address) = 0;

        virtual bool writeEeprom(uint32_t address, uint8_t data) = 0;

        virtual uint8_t readFuse(uint8_t address) = 0;

        virtual bool writeFuse(uint8_t address, uint8_t data) = 0;

        virtual uint8_t readLockBits() = 0;

        virtual uint8_t readCalibration() = 0;

        virtual const uint8_t *readSignature() = 0;
    };

    class Device {
        ATPR *handle;

    public:
        explicit Device(ATPR *handle);

        std::unique_ptr<Program> program(Protocol protocol);
    };
}
#endif // __ATPRSUP_AVR_H__
