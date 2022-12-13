use autocxx::prelude::*;
use autocxx::c_int;

include_cpp! {
    #include "hooks/hooks.h"
    #include "dhcp/pkt4.h"
    generate!("isc::hooks::LibraryHandle")
    generate!("isc::hooks::CalloutHandle")
    generate!("isc::dhcp::Pkt4Ptr")
    safety!(unsafe)
}

#[no_mangle]
pub extern "C" fn version() -> c_int {
    // I'm unable to get the constant to generate, so for now, hard code the value
    return c_int::from(20301); // 2.3.1
}

#[no_mangle]
pub extern "C" fn load(_handle: ffi::isc::hooks::LibraryHandle) -> c_int {
    // Load the hook here

    return c_int::from(0);
}

#[no_mangle]
pub extern "C" fn pkt4_receive(handle: ffi::isc::hooks::CalloutHandle) -> c_int {
    // Do processing here

    return c_int::from(0)
}