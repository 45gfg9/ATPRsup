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

#include "avr/isp.h"
#include <cassert>

using namespace AVR;

// TODO

ISP::ISP(ATPR *handle, uint8_t ispSpeed) : Program(handle) {
    assert(ispSpeed < 14);

    const uint8_t map[6] = {0b1000, 0b1101, 0b1001, 0b1110, 0b1010, 0b1011};
    this->ispSpeed = ispSpeed < 6 ? map[ispSpeed] : ispSpeed - 6;

    handle->write(0x00, 0x0000, 0x0000, nullptr, 0);
}

ISP::~ISP() = default;

bool ISP::begin() {
    uint8_t resp;

    handle->read(0x00, ispSpeed, 0x0000, &resp, 1);

    return resp;
}

bool ISP::chipErase() {
    return false;
}

bool ISP::isReady() {
    return false;
}

bool ISP::latchData() {
    return false;
}

uint8_t ISP::readFlash(uint32_t address) {
    return 0;
}

bool ISP::writeFlash(uint32_t address, uint8_t data) {
    return false;
}

bool ISP::loadFlashPage(uint32_t address, const uint8_t *data, uint16_t pageSize) {
    return false;
}

bool ISP::flushFlashPage() {
    return false;
}

uint8_t ISP::readEeprom(uint32_t address) {
    return 0;
}

bool ISP::writeEeprom(uint32_t address, uint8_t data) {
    return false;
}

uint8_t ISP::readFuse(uint8_t address) {
    return 0;
}

bool ISP::writeFuse(uint8_t address, uint8_t data) {
    return false;
}

uint8_t ISP::readLockBits() {
    return 0;
}

const uint8_t *ISP::readSignature() {
    uint8_t buf[4];
    for (uint8_t i = 0; i < 3; i++) {
        handle->read(0x01, 0x0030, i, buf, 4);

        sigBuf[i] = buf[3];
    }

    return sigBuf;
}
