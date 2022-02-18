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

#include "avr/avr.h"
#include "avr/isp.h"

using namespace AVR;

Program::Program(ATPR *handle) : handle(handle) {
}

Program::~Program() {
    uint8_t buf[4];
    handle->read(0x00, 0x9140, 0x469c, buf, 4);
    assert(*reinterpret_cast<uint32_t *>(buf) == 0x469c9140);
}

Device::Device(ATPR *handle) : handle(handle) {
}

std::unique_ptr<Program> Device::program(Protocol protocol) {
    switch (protocol) {
        case Protocol::ISP:
            return std::make_unique<ISP>(handle);
        default:
            assert(false);
    }
}
