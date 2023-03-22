#[cfg_attr(target_arch = "wasm32", export_name = "open")]
#[no_mangle]
pub fn open(path: *const c_char, _: c_int, _: c_int) -> c_int {
    let inode;
    let cstr = unsafe { CString::from_raw(path as *mut c_char) };
    let path_buf = PathBuf::from(cstr.to_str().expect("Failed to convert CString to str"));

    {
        let fs = unsafe { &mut *FILESYSTEM.as_ref().unwrap().get() };
        inode = fs.lookup_inode(&path_buf).expect("Failed to find the inode for the given path");
    }


    let fd;
    {
        let process = unsafe { &mut *PROCESS.as_ref().unwrap().get() };
        fd = process.open(unsafe { &mut *FILESYSTEM.as_ref().unwrap().get() }, inode).expect("Failed to open the file");
    }
    return fd as c_int;
}

#[cfg_attr(target_arch = "wasm32", export_name = "close")]
#[no_mangle]
pub fn close(fd: c_int) -> c_int {
    let process = unsafe { &mut *PROCESS.as_ref().unwrap().get() };
    let close_result = process.close(fd);
    match close_result {
        Ok(_) => return 0,
        Err(_) => return -1,
    };
}


use std::{ptr, slice};

#[no_mangle]
pub extern "C" fn read(fd: c_int, buffer_pointer: *mut c_void, size: usize) -> isize {
    let process = unsafe { &mut *PROCESS.as_ref().unwrap().get() };

     unsafe {
        let buf = slice::from_raw_parts_mut(buffer_pointer as *mut u8, size);



        let bytes_read = process.read(fd, buf).expect("Failed to read from the file");
        return bytes_read as isize
     }
}

//alias bc no libc for wasm targets :/
//use libc::{ssize_t, size_t};
type size_t  = usize;
type ssize_t  = isize;

//The write function returns the number of bytes successfully written into the file,
//which may at times be less than the specified nbytes.
//returns -1 if an exceptional condition is encountered
#[no_mangle]
pub extern "C" fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t {
    let process = unsafe { &mut *PROCESS.as_ref().unwrap().get() };
    let buffer = unsafe { slice::from_raw_parts(buf as *const u8, count) };

    let bytes_written = process.write(fd, &*buffer).expect("failed to write to the file");

    //TODO: this, but better
    //if let Some(Err) = err {
         //set errno
        //return -1
    //}
    return bytes_written as isize
}

