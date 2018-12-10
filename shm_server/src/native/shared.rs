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
const IPC_RMID: c_int = 10;
#[cfg(target_os = "solaris")]
extern "C" {
    fn shmget(key: key_t, size: size_t, flags: c_int) -> c_int;
    fn shmat(id: c_int, address: *mut c_void, flags: c_int) -> *mut c_void;
    fn shmdt(address: *mut c_void) -> c_int;
    fn shmctl(id: c_int, cmd: c_int, address: *mut c_void) -> c_int;
}
#[cfg(not(target_os = "solaris"))]
use libc::{shmctl, shmget, shmdt, shmat, IPC_CREAT, IPC_EXCL, IPC_RMID};

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

pub fn detach_mem<T>(address: *mut T) -> Result<(), Errno> {
    let address = address as *mut _ as *mut c_void;
    if unsafe { shmdt(address) } == 0 {
        Ok(())
    } else {
        Err(Errno::current())
    }
}

pub fn remove_mem(id: c_int) -> Result<(), Errno> {
    if unsafe { shmctl(id, IPC_RMID, null_mut()) } == 0 {
        Ok(())
    } else {
        Err(Errno::current())
    }
}
