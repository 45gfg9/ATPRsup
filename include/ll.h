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

#ifndef __ATPRSUP_LL_H__
#define __ATPRSUP_LL_H__

#include <memory>
#include <cstdint>

#include <libusb-1.0/libusb.h>

// Shared VID/PID pair provided by Objective Development
#define ATPR_VID 0x16c0
#define ATPR_PID 0x05dc
#define ATPR_VENDOR_NAME "45gfg9.net"
#define ATPR_PRODUCT_NAME "ATPR"

namespace AVR {
    class Device;
}

namespace AT89 {
    class AT89Device;
}

namespace NRF24LE1 {
    class NRF24LE1Device;
}
extern libusb_context *ATPRContext;

libusb_device_handle *openDevice(uint16_t vid, const char *vendorName, uint16_t pid, const char *productName);

class ATPR {
    static const uint8_t REQ_TYPE_CTRL_IN = LIBUSB_REQUEST_TYPE_VENDOR | LIBUSB_RECIPIENT_DEVICE | LIBUSB_ENDPOINT_IN;
    static const uint8_t REQ_TYPE_CTRL_OUT = LIBUSB_REQUEST_TYPE_VENDOR | LIBUSB_RECIPIENT_DEVICE | LIBUSB_ENDPOINT_OUT;

    static const uint32_t TIMEOUT = 1000; // ms

    libusb_device_handle *handle;

    mutable uint8_t verBuf[3];
public:
    explicit ATPR(libusb_device_handle *handle);
    ~ATPR();

    uint32_t read(uint8_t request, uint16_t value, uint16_t index, uint8_t *data, uint16_t len);
    uint32_t write(uint8_t request, uint16_t value, uint16_t index, uint8_t *data, uint16_t len);

    const uint8_t *version();

    std::unique_ptr<AVR::Device> openAVR();
};

#endif // __ATPRSUP_LL_H__
