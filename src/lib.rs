//Some generic libc bindings
//
use core::ffi::c_ulong;
//alias bc no libc for wasm targets :/
// a la libc::{c_char, c_int, c_long, c_void, off_t, size_t, ssize_t, timespec, time_t};
// -- TODO: hard confirm type patching
type size_t  = usize;
type ssize_t  = isize;
type c_char  = usize;
type c_int  = isize;
type c_long  = usize;
type c_void  = isize;
type off_t  = usize;
type timespec  = isize;
type time_t  = isize;
type tv_sec  = isize;
type tv_usec  = isize;

//don't have the system to take care of it
pub struct timeval {
    time_t:     tv_sec,     // seconds,
    suseconds_t: tv_usec    // microseconds
}

// File I/O
// opens a file or device and returns a file descriptor
#[cfg_attr(target_arch = "wasm32", export_name = "open")] // can add apply to all as needed
#[no_mangle]
pub extern "C" fn open(path: *const c_char, oflag: c_int, mode: c_int) -> c_int {
    unimplemented!();
}

// creates a file and returns a file descriptor
pub extern "C" fn creat(path: *const c_char, mode: c_int) -> c_int {
    unimplemented!();
}

// closes a file descriptor
pub extern "C" fn close(fd: c_int) -> c_int {
    unimplemented!();
}

// reads data from a file descriptor into a buffer
pub extern "C" fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t {
    unimplemented!();
}

// writes data from a buffer to a file descriptor
pub extern "C" fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t {
    unimplemented!();
}

// changes the position of the file offset of a file descriptor
pub extern "C" fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t {
    unimplemented!();
}

// opens a directory stream and returns a pointer to a directory stream object
pub extern "C" fn opendir(dirname: *const c_char) -> *mut c_void {
    unimplemented!();
}

// reads a directory stream and returns a pointer to a directory entry object
pub extern "C" fn readdir(dirp: *mut c_void) -> *mut c_void {
    unimplemented!();
}

// closes a directory stream
pub extern "C" fn closedir(dirp: *mut c_void) -> c_int {
    unimplemented!();
}

// retrieves information about a file or symbolic link
pub extern "C" fn stat(path: *const c_char, buf: *mut c_void) -> c_int {
    unimplemented!();
}

// retrieves information about a file descriptor
pub extern "C" fn fstat(fd: c_int, buf: *mut c_void) -> c_int {
    unimplemented!();
}

// retrieves information about a symbolic link
pub extern "C" fn lstat(path: *const c_char, buf: *mut c_void) -> c_int {
    unimplemented!();
}

// changes the permission bits of a file
pub extern "C" fn chmod(path: *const c_char, mode: c_int) -> c_int {
    unimplemented!();
}

// changes the permission bits of a file descriptor
pub extern "C" fn fchmod(fd: c_int, mode: c_int) -> c_int {
    unimplemented!();
}

// creates a hard link to a file
pub extern "C" fn link(existing: *const c_char, new: *const c_char) -> c_int {
    unimplemented!();
}

// deletes a name from the filesystem
pub extern "C" fn unlink(path: *const c_char) -> c_int {
    unimplemented!();
}

// creates a new directory
pub extern "C" fn mkdir(path: *const c_char, mode: c_int) -> c_int {
    unimplemented!();
}

// removes a directory
pub extern "C" fn rmdir(path: *const c_char) -> c_int {
    unimplemented!();
}

// changes the current working directory
pub extern "C" fn chdir(path: *const c_char) -> c_int {
    unimplemented!();
}

// gets the current working directory
pub extern "C" fn getcwd(buf: *mut c_char, size: size_t) -> *mut c_char {
    unimplemented!();
}

// truncates a file to a specified length
pub extern "C" fn truncate(path: *const c_char, length: off_t) -> c_int {
    unimplemented!();
}

// truncates a file descriptor to a specified length
pub extern "C" fn ftruncate(fd: c_int, length: off_t) -> c_int {
    unimplemented!();
}

// synchronizes a file's in-core state with that on disk
pub extern "C" fn fsync(fd: c_int) -> c_int {
    unimplemented!();
}

// synchronizes a file's data in-core with that on disk
pub extern "C" fn fdatasync(fd: c_int) -> c_int {
    unimplemented!();
}

// synchronizes the system's in-core state with that on disk
pub extern "C" fn sync() -> c_void {
    unimplemented!();
}

// changes file timestamps
pub extern "C" fn utime(path: *const c_char, times: *const c_void) -> c_int {
    unimplemented!();
}

// changes file timestamps with microsecond granularity
pub extern "C" fn utimes(path: *const c_char, times: *const timeval) -> c_int {
    unimplemented!();
}

// changes file timestamps with microsecond granularity for a file descriptor
pub extern "C" fn futimes(fd: c_int, times: *const timeval) -> c_int {
    unimplemented!();
}

// changes file timestamps with nanosecond granularity for a file descriptor
pub extern "C" fn futimens(fd: c_int, times: *const timespec) -> c_int {
    unimplemented!();
}

// changes file timestamps with nanosecond granularity for a file or directory
pub extern "C" fn utimensat(dirfd: c_int, path: *const c_char, times: *const timespec, flags: c_int) -> c_int {
    unimplemented!();
}

// locks a file region or an entire file
pub extern "C" fn flock(fd: c_int, operation: c_int) -> c_int {
    unimplemented!();
}

// lock a file region or an entire file
pub extern "C" fn lockf(fd: c_int, cmd: c_int, len: off_t) -> c_int {
    unimplemented!();
}

// creates a symbolic link to a file
pub extern "C" fn symlink(target: *const c_char, linkpath: *const c_char) -> c_int {
    unimplemented!();
}

// reads the value of a symbolic link
pub extern "C" fn readlink(path: *const c_char, buf: *mut c_char, bufsize: size_t) -> ssize_t {
    unimplemented!();
}

// resolves a relative or symbolic path name to an absolute path name
pub extern "C" fn realpath(path: *const c_char, resolved_path: *mut c_char) -> *mut c_char {
    unimplemented!();
}

// retrieves information about a mounted filesystem
pub extern "C" fn statvfs(path: *const c_char, buf: *mut c_void) -> c_int {
    unimplemented!();
}

// retrieves information about a mounted filesystem for a file descriptor
pub extern "C" fn fstatvfs(fd: c_int, buf: *mut c_void) -> c_int {
    unimplemented!();
}

// checks whether the calling process can access a file
pub extern "C" fn access(path: *const c_char, mode: c_int) -> c_int {
    unimplemented!();
}

// duplicates a file descriptor
pub extern "C" fn dup(fd: c_int) -> c_int {
    unimplemented!();
}

// duplicates a file descriptor to a specified descriptor number
pub extern "C" fn dup2(oldfd: c_int, newfd: c_int) -> c_int {
    unimplemented!();
}

// creates a pipe
pub extern "C" fn pipe(pipefd: *mut c_int) -> c_int {
    unimplemented!();
}

// performs I/O control operations on a file descriptor
pub extern "C" fn ioctl(fd: c_int, request: c_long) -> c_int {
    unimplemented!();
}

// mounts a filesystem
pub extern "C" fn mount(src: *const c_char, target: *const c_char, fstype: *const c_char, flags: c_ulong, data: *const c_void) -> c_int {
    unimplemented!();
}

// unmounts a mounted filesystem
pub extern "C" fn umount(target: *const c_char) -> c_int {
    unimplemented!();
}
