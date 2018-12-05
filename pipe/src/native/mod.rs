use libc::{fork, pipe, dup2, execlp, c_char};

mod errno;
mod strings;

use std::ffi::{CString, NulError};
use self::errno::Errno;
use std::os::unix::io::{RawFd, AsRawFd};
use std::fs::File;
use std::ptr::null;

pub fn execute_in_child<T: FnOnce() -> Errno>(actions: T) -> Result<(), Errno> {
    match unsafe { fork() } {
        -1 => Err(Errno::current()),
        0 => Err(actions()),
        _ => Ok(()),
    }
}

pub fn create_pipe() -> Result<(RawFd, RawFd), Errno> {
    let mut descriptors: [RawFd; 2] = [0; 2];
    let status = unsafe { pipe(descriptors.as_mut_ptr()) };
    if status == 0 {
        Ok((descriptors[0], descriptors[1]))
    } else {
        Err(Errno::current())
    }
}

pub fn as_stdin(file: File) -> Result<(), Errno> {
    let fd = file.as_raw_fd();
    let status = unsafe { dup2(fd, 0) };
    if status == -1 {
        Err(Errno::current())
    } else {
        Ok(())
    }
}

pub fn execute(name: &str) -> Result<Errno, NulError> {
    let nul_terminated = CString::new(name)?;
    unsafe {
        execlp(
            nul_terminated.as_ptr(),
            nul_terminated.as_ptr(),
            null::<c_char>(),
        );
    }
    Ok(Errno::current())
}
