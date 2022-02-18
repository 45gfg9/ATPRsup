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

#include "ll.h"
#include "avr/avr.h"

#include <cstring>
#include <cassert>

libusb_context *ATPRContext;

libusb_device_handle *openDevice(uint16_t vid, const char *vendorName, uint16_t pid, const char *productName) {
    libusb_device_handle *handle = nullptr;
    libusb_device **devList;

    ssize_t devListLen = libusb_get_device_list(ATPRContext, &devList);

    for (ssize_t i = 0; i < devListLen; i++) {
        libusb_device *device = devList[i];
        libusb_device_descriptor descriptor;
        libusb_get_device_descriptor(device, &descriptor);

        if (descriptor.idVendor != vid || descriptor.idProduct != pid) {
            continue;
        }

        libusb_open(device, &handle);
        if (!handle) {
            // error opening device
            continue;
        }

        const size_t BUFSIZE = 256;
        char buf[BUFSIZE];
        int e;

        // TODO error handling
        e = libusb_get_string_descriptor_ascii(handle, descriptor.iManufacturer, reinterpret_cast<unsigned char *>(buf),
                                               BUFSIZE);
        assert(e >= 0);
        if (strcmp(buf, vendorName) != 0) {
            // not match
            libusb_close(handle);
            handle = nullptr;
            continue;
        }
        e = libusb_get_string_descriptor_ascii(handle, descriptor.iProduct, reinterpret_cast<unsigned char *>(buf),
                                               BUFSIZE);
        assert(e >= 0);
        if (strcmp(buf, productName) != 0) {
            // not match
            libusb_close(handle);
            handle = nullptr;
            continue;
        }

        break;
    }

    libusb_free_device_list(devList, true);
    return handle;
}

ATPR::ATPR(libusb_device_handle *handle) : handle(handle) {
    verBuf[0] = 0xFF;
}

ATPR::~ATPR() {
    libusb_close(handle);
    handle = nullptr;
}

uint32_t ATPR::read(uint8_t request, uint16_t value, uint16_t index, uint8_t *data, uint16_t len) {
    return libusb_control_transfer(handle, REQ_TYPE_CTRL_IN, request, value, index, data, len, TIMEOUT);
}

uint32_t ATPR::write(uint8_t request, uint16_t value, uint16_t index, uint8_t *data, uint16_t len) {
    return libusb_control_transfer(handle, REQ_TYPE_CTRL_OUT, request, value, index, data, len, TIMEOUT);
}

const uint8_t *ATPR::version() {
    if (verBuf[0] == 0xFF) {
        uint8_t stamp = time(nullptr);

        uint8_t buf[4];
        auto ret = read(0xFF, stamp, 0, buf, 4);
        printf("%d\n", ret);

        assert(buf[0] == stamp);

        memcpy(verBuf, buf + 1, 3);
    }

    return verBuf;
}

std::unique_ptr<AVR::Device> ATPR::openAVR() {
    return std::make_unique<AVR::Device>(this);
}
