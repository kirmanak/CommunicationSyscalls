use libc::{getpid, getuid, getgid, getloadavg, time, c_int};

mod errno;
mod strings;
pub mod shared;

use self::errno::Errno;
use std::ptr::null_mut;

/// Gets the identificator of the current process
pub fn get_pid() -> u32 {
    unsafe { getpid() as u32 }
}

/// Gets the identificator of the user who is executing the current process
pub fn get_uid() -> u32 {
    unsafe { getuid() }
}

/// Gets the identificator of the group which is executing the current process
pub fn get_gid() -> u32 {
    unsafe { getgid() }
}

/// Requests average load of the system during the last 1 minute, 5 minutes, 15 minutes.
/// # Panics
/// If getloadavg() returned different numbers of results
pub fn get_loadavg() -> Result<Vec<f64>, Errno> {
    const AMOUNT: c_int = 3;
    let mut buffer = Vec::with_capacity(AMOUNT as usize);
    match unsafe { getloadavg(buffer.as_mut_ptr(), AMOUNT as c_int) } {
        -1 => Err(Errno::current()),
        AMOUNT => {
            unsafe {
                buffer.set_len(AMOUNT as usize);
            }
            Ok(buffer)
        }
        _ => panic!("getloadavg() has successfully returned an unexpected value."),
    }
}

pub fn get_time() -> i64 {
    unsafe { time(null_mut()) }
}
