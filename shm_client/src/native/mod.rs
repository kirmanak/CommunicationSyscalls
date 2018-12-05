use libc::{key_t, c_void, c_int};

mod errno;
mod strings;

use self::errno::Errno;
use std::ptr::null_mut;

const KEY: key_t = 225072;

#[cfg(target_os = "solaris")]
use libc::size_t;
#[cfg(target_os = "solaris")]
const SHM_RDONLY: c_int = 0o10000;
#[cfg(target_os = "solaris")]
extern "C" {
    fn shmget(key: key_t, size: size_t, flags: c_int) -> c_int;
    fn shmat(id: c_int, address: *mut c_void, flags: c_int) -> *mut c_void;
}
#[cfg(not(target_os = "solaris"))]
use libc::{shmget, shmat, SHM_RDONLY};

/// Calls shmget without flags to get id of the shared memory segment
pub fn allocate_mem(size: usize) -> Result<c_int, Errno> {
    let status = unsafe { shmget(KEY, size, 0) };
    if status == -1 {
        Err(Errno::current())
    } else {
        Ok(status)
    }
}

/// Calls shmat with read-only flag to attach the shared memory segment to the current process
pub fn attach_mem(id: c_int) -> Result<*mut c_void, Errno> {
    let address = null_mut();
    let status = unsafe { shmat(id, address, SHM_RDONLY) };
    if status == unsafe { address.offset(-1) } {
        Err(Errno::current())
    } else {
        Ok(status)
    }
}
