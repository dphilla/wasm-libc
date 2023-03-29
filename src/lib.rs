//Generic libc bindings

use libc::{c_char, c_int, c_long, c_void, off_t, size_t, ssize_t, timespec, time_t};

//alias bc no libc for wasm targets :/
type size_t  = usize;
type ssize_t  = isize;
// etc.

// File I/O
#[cfg_attr(target_arch = "wasm32", export_name = "open")] // can add apply to all as needed p
#[no_mangle]
pub extern "C" fn open(path: *const c_char, oflag: c_int, mode: c_int) -> c_int;

#[no_mangle]
pub extern "C" fn creat(path: *const c_char, mode: c_int) -> c_int;

#[no_mangle]
pub extern "C" fn close(fd: c_int) -> c_int;

#[no_mangle]
pub extern "C" fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;

#[no_mangle]
pub extern "C" fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;

#[no_mangle]
pub extern "C" fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t;

// Directories
#[no_mangle]
pub extern "C" fn opendir(dirname: *const c_char) -> *mut c_void;

#[no_mangle]
pub extern "C" fn readdir(dirp: *mut c_void) -> *mut c_void;

#[no_mangle]
pub extern "C" fn closedir(dirp: *mut c_void) -> c_int;

// File status
#[no_mangle]
pub extern "C" fn stat(path: *const c_char, buf: *mut c_void) -> c_int;

#[no_mangle]
pub extern "C" fn fstat(fd: c_int, buf: *mut c_void) -> c_int;

#[no_mangle]
pub extern "C" fn lstat(path: *const c_char, buf: *mut c_void) -> c_int;

// File permissions
#[no_mangle]
pub extern "C" fn chmod(path: *const c_char, mode: c_int) -> c_int;

#[no_mangle]
pub extern "C" fn fchmod(fd: c_int, mode: c_int) -> c_int;

// File links
#[no_mangle]
pub extern "C" fn link(existing: *const c_char, new: *const c_char) -> c_int;

#[no_mangle]
pub extern "C" fn unlink(path: *const c_char) -> c_int;

// Filesystem
#[no_mangle]
pub extern "C" fn mkdir(path: *const c_char, mode: c_int) -> c_int;

#[no_mangle]
pub extern "C" fn rmdir(path: *const c_char) -> c_int;

#[no_mangle]
pub extern "C" fn chdir(path: *const c_char) -> c_int;

#[no_mangle]
pub extern "C" fn getcwd(buf: *mut c_char, size: size_t) -> *mut c_char;

#[no_mangle]
pub extern "C" fn truncate(path: *const c_char, length: off_t) -> c_int;

#[no_mangle]
pub extern "C" fn ftruncate(fd: c_int, length: off_t) -> c_int;

// Synchronization
#[no_mangle]
pub extern "C" fn fsync(fd: c_int) -> c_int;

#[no_mangle]
pub extern "C" fn fdatasync(fd: c_int) -> c_int;

#[no_mangle]
pub extern "C" fn sync() -> c_void;

// File timestamps
#[no_mangle]
pub extern "C" fn utime(path: *const c_char, times: *const c_void) -> c_int;

#[no_mangle]
pub extern "C" fn utimes(path: *const c_char, times: *const timeval) -> c_int;

#[no_mangle]
pub extern "C" fn futimes(fd: c_int, times: *const timeval) -> c_int;

#[no_mangle]
pub extern "C" fn futimens(fd: c_int, times: *const timespec) -> c_int;

#[no_mangle]
pub extern "C" fn utimensat(dirfd: c_int, path: *const c_char, times: *const timespec, flags: c_int) -> c_int;

// File locking
#[no_mangle]
pub extern "C" fn flock(fd: c_int, operation: c_int) -> c_int;

#[no_mangle]
pub extern "C" fn lockf(fd: c_int, cmd: c_int, len: off_t) -> c_int;

// Symbolic links
#[no_mangle]
pub extern "C" fn symlink(target: *const c_char, linkpath: *const c_char) -> c_int;

#[no_mangle]
pub extern "C" fn readlink(path: *const c_char, buf: *mut c_char, bufsize: size_t) -> ssize_t;

// Path manipulation
#[no_mangle]
pub extern "C" fn realpath(path: *const c_char, resolved_path: *mut c_char) -> *mut c_char;

// Filesystem space
#[no_mangle]
pub extern "C" fn statvfs(path: *const c_char, buf: *mut c_void) -> c_int;

#[no_mangle]
pub extern "C" fn fstatvfs(fd: c_int, buf: *mut c_void) -> c_int;

// Access control
#[no_mangle]
pub extern "C" fn access(path: *const c_char, mode: c_int) -> c_int;

// Process file control
#[no_mangle]
pub extern "C" fn dup(fd: c_int) -> c_int;

#[no_mangle]
pub extern "C" fn dup2(oldfd: c_int, newfd: c_int) -> c_int;

#[no_mangle]
pub extern "C" fn pipe(pipefd: *mut c_int) -> c_int;

// System file control
#[no_mangle]
pub extern "C" fn ioctl(fd: c_int, request: c_long, ...) -> c_int;

// Filesystem mount
#[no_mangle]
pub extern "C" fn mount(src: *const c_char, target: *const c_char, fstype: *const c_char, flags: c_ulong, data: *const c_void) -> c_int;

#[no_mangle]
pub extern "C" fn umount(target: *const c_char) -> c_int;
