mod errno;
mod strings;

use self::errno::Errno;

use libc::{key_t, c_void, c_int};

use std::mem::{uninitialized, size_of};

const KEY: key_t = 225072;

#[cfg(target_os = "solaris")]
use libc::{ssize_t, size_t, c_long};
#[cfg(target_os = "solaris")]
extern "C" {
    fn msgget(key: key_t, flags: c_int) -> c_int;
    fn msgrcv(
        msqid: c_int,
        msgp: *mut c_void,
        msgsz: size_t,
        msgtyp: c_long,
        msgflg: c_int,
    ) -> ssize_t;
}
#[cfg(not(target_os = "solaris"))]
use libc::{msgget, msgrcv};

/// Calls shmget with flags IPC_CREAT and IPC_EXCL to allocate a new shared memory segment
pub fn get_queue() -> Result<c_int, Errno> {
    let status = unsafe { msgget(KEY, 0) };
    if status == -1 {
        Err(Errno::current())
    } else {
        Ok(status)
    }
}

/// Calls shmat without flags to attach the shared memory segment to the current process
pub fn get_message<T>(id: c_int) -> Result<T, Errno> {
    let size = size_of::<T>();
    let buffer: T = unsafe { uninitialized() };
    let pointer = Box::into_raw(Box::new(buffer));
    let status = unsafe { msgrcv(id, pointer as *mut c_void, size, 0, 0) };
    let buffer = unsafe { Box::from_raw(pointer) };
    let buffer = *buffer;
    if status == -1 {
        Err(Errno::current())
    } else {
        Ok(buffer)
    }
}
