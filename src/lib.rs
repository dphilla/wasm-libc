#[cfg_attr(target_arch = "wasm32", export_name = "open")]
#[no_mangle]
pub fn open(path: *const c_char, _: c_int, _: c_int) -> c_int {
    // implement open and related syscalls
}

#[cfg_attr(target_arch = "wasm32", export_name = "close")]
#[no_mangle]
pub fn close(fd: c_int) -> c_int {
    // implement close and related syscalls
}


use std::{ptr, slice};

#[cfg_attr(target_arch = "wasm32", export_name = "read")]
#[no_mangle]
pub extern "C" fn read(fd: c_int, buffer_pointer: *mut c_void, size: usize) -> isize {
    // implement read and related syscalls
}

//alias bc no libc for wasm targets :/
//use libc::{ssize_t, size_t};
type size_t  = usize;
type ssize_t  = isize;

#[cfg_attr(target_arch = "wasm32", export_name = "write")]
#[no_mangle]
pub extern "C" fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t {
    // implement write and related syscalls
}

