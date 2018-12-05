use libc::{key_t, c_void, c_int};

use native::errno::Errno;
use std::mem::size_of;
use std::fmt::Debug;

const KEY: key_t = 225072;

#[cfg(target_os = "solaris")]
const IPC_CREAT: c_int = 0o1000;
#[cfg(target_os = "solaris")]
const IPC_EXCL: c_int = 0o2000;
#[cfg(target_os = "solaris")]
use libc::size_t;
#[cfg(target_os = "solaris")]
extern "C" {
    fn msgget(key: key_t, flags: c_int) -> c_int;
    fn msgsnd(id: c_int, address: *const c_void, size: size_t, flags: c_int) -> c_int;
}
#[cfg(not(target_os = "solaris"))]
use libc::{msgget, msgsnd, IPC_CREAT, IPC_EXCL};

/// Calls shmget with flags IPC_CREAT and IPC_EXCL to allocate a new shared memory segment
pub fn get_queue() -> Result<c_int, Errno> {
    let status = unsafe { msgget(KEY, IPC_CREAT | IPC_EXCL | 0o600) };
    if status == -1 {
        Err(Errno::current())
    } else {
        Ok(status)
    }
}

/// Calls shmat without flags to attach the shared memory segment to the current process
pub fn send_message<T: Debug>(id: c_int, value: Box<T>) -> Result<(), Errno> {
    let size = size_of::<T>();
    let pointer = Box::into_raw(value) as *const c_void;
    let status = unsafe { msgsnd(id, pointer, size, 0) };
    if status == -1 {
        Err(Errno::current())
    } else {
        Ok(())
    }
}
