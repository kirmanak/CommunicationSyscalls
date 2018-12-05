use libc::{key_t, c_void, c_int};

use native::errno::Errno;
use std::ptr::null_mut;

const KEY: key_t = 225072;

#[cfg(target_os = "solaris")]
use libc::size_t;
#[cfg(target_os = "solaris")]
const IPC_CREAT: c_int = 0o1000;
#[cfg(target_os = "solaris")]
const IPC_EXCL: c_int = 0o2000;
#[cfg(target_os = "solaris")]
extern "C" {
    fn shmget(key: key_t, size: size_t, flags: c_int) -> c_int;
    fn shmat(id: c_int, address: *mut c_void, flags: c_int) -> *mut c_void;
}
#[cfg(not(target_os = "solaris"))]
use libc::{shmget, shmat, IPC_CREAT, IPC_EXCL};

/// Calls shmget with flags IPC_CREAT and IPC_EXCL to allocate a new shared memory segment
pub fn allocate_mem(size: usize) -> Result<c_int, Errno> {
    let status = unsafe { shmget(KEY, size, IPC_CREAT | IPC_EXCL | 0o600) };
    if status == -1 {
        Err(Errno::current())
    } else {
        Ok(status)
    }
}

/// Calls shmat without flags to attach the shared memory segment to the current process
pub fn attach_mem(id: c_int) -> Result<*mut c_void, Errno> {
    let address = null_mut();
    let status = unsafe { shmat(id, address, 0) };
    if status == unsafe { address.offset(-1) } {
        Err(Errno::current())
    } else {
        Ok(status)
    }
}
