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

#include "ATPRsup.h"
#include "ll.h"

#include <iostream>

#include <libusb-1.0/libusb.h>


int main() {
    std::cout << "Hello ATPRsup" << std::endl;
    libusb_init(&ATPRContext);

    // TODO
    ATPR device(openDevice(ATPR_VID, ATPR_VENDOR_NAME, ATPR_PID, ATPR_PRODUCT_NAME));
    auto ver = device.version();
    printf("Firmware version %u.%u.%u\n", ver[0], ver[1], ver[2]);

    auto avr = device.openAVR();
    auto isp = avr->program(AVR::Protocol::ISP);
    isp->begin();
    auto signature = isp->readSignature();

    printf("Target signature %02X %02X %02X\n", signature[0], signature[1], signature[2]);

    std::atexit([] {
        libusb_exit(ATPRContext);
        ATPRContext = nullptr;
    });

    return 0;
}
