mod errno;
mod strings;

use libc::{mmap, PROT_READ, MAP_SHARED, MAP_FAILED};

use native::errno::Errno;
use std::ptr::null_mut;

use std::os::unix::io::RawFd;

/// Creates file with size of at least one instance of T
pub fn open_file<T>(fd: RawFd, size: usize) -> Result<*mut T, Errno> {
    let offset = 0;
    let result = unsafe { mmap(null_mut(), size, PROT_READ, MAP_SHARED, fd, offset) };
    if result == MAP_FAILED {
        Err(Errno::current())
    } else {
        Ok(result as *mut T)
    }
}
