cmake_minimum_required(VERSION 3.16)

project(snap7-rust-bindings LANGUAGES C CXX)

set(CMAKE_C_STANDARD 99)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

list(APPEND S7SRC
    src/sys/snap_msgsock.cpp
    src/sys/snap_msgsock.h
    src/sys/snap_platform.h
    src/sys/snap_sysutils.cpp
    src/sys/snap_sysutils.h
    src/sys/snap_tcpsrvr.cpp
    src/sys/snap_tcpsrvr.h
    src/sys/snap_threads.cpp
    src/sys/snap_threads.h
    src/core/s7_client.cpp
    src/core/s7_client.h
    src/core/s7_firmware.h
    src/core/s7_isotcp.cpp
    src/core/s7_isotcp.h
    src/core/s7_micro_client.cpp
    src/core/s7_micro_client.h
    src/core/s7_partner.cpp
    src/core/s7_partner.h
    src/core/s7_peer.cpp
    src/core/s7_peer.h
    src/core/s7_server.cpp
    src/core/s7_server.h
    src/core/s7_text.cpp
    src/core/s7_text.h
    src/core/s7_types.h
    src/lib/snap7_libmain.cpp
    src/lib/snap7_libmain.h
)

if(WIN32)
    list(APPEND S7SRC
        src/sys/win_threads.h
        src/lib/snap7.def
    )
elseif(UNIX)
    list(APPEND S7SRC
        src/sys/unix_threads.h
    )
else()
    message(FATAL_ERROR "Unsupported platform")
endif()

add_library(snap7 STATIC ${S7SRC})
target_include_directories(snap7 PRIVATE src/core src/sys src/lib)
if (WIN32)
    target_link_libraries(snap7 PRIVATE Winmm Ws2_32)
endif()

install(TARGETS snap7
    LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
    ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR})
