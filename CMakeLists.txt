cmake_minimum_required(VERSION 3.17)
project(ATPRsup)

set(CMAKE_CXX_STANDARD 20)
set(EXECUTABLE_OUTPUT_PATH ${PROJECT_SOURCE_DIR}/bin)

include_directories(include)

INCLUDE_DIRECTORIES(/usr/local/opt/libusb/include)
LINK_DIRECTORIES(/usr/local/opt/libusb/lib)
LINK_LIBRARIES(usb-1.0)

aux_source_directory(src SRC_LIST)
aux_source_directory(src/avr SRC_LIST)
add_executable(ATPRsup ${SRC_LIST})

target_link_libraries(ATPRsup usb-1.0)
