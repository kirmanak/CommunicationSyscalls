use libc::{c_int, strerror};

use std::fmt;

use native::strings::copy_raw;

#[cfg(target_os = "solaris")]
unsafe fn errno() -> *const c_int {
    libc::___errno()
}

#[cfg(not(target_os = "solaris"))]
unsafe fn errno() -> *const c_int {
    libc::__errno_location()
}

/// Represents the state of errno variable
pub struct Errno {
    pub code: c_int,
    info: String,
}

impl Errno {
    /// Returns the current state of errno variable
    /// # Panics
    /// If errno location is undefined or errno state is unknown
    /// or strerror() result contains non-unicode characters
    pub fn current() -> Self {
        let pointer = unsafe { errno() };
        if pointer.is_null() {
            panic!("Errno location is undefined!");
        }
        let code = unsafe { *pointer };
        let info = unsafe { strerror(code) };
        if info.is_null() {
            panic!("Errno state: {} is unknown!", code);
        }
        let info = unsafe { copy_raw(info) };
        Errno { code, info }
    }
}

impl fmt::Debug for Errno {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Errno code: {}, strerror: {}", self.code, self.info)
    }
}
