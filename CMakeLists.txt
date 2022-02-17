cmake_minimum_required(VERSION 3.20)
project(ATPRsup)

set(CMAKE_CXX_STANDARD 20)

include_directories(include)

find_library(LIBUSB NAMES usb)

add_subdirectory(src)
