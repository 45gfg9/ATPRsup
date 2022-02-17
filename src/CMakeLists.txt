add_executable(ATPRsup
        main.cpp
        ATPRsup.h
        ll.h)

target_link_libraries(ATPRsup ${LIBUSB})
