pub use libc::c_int;

use libc::{signal, sighandler_t, SIG_ERR};

use super::errno::Errno;
use std::mem::transmute;

pub fn register_handler(number: c_int, handler: fn(c_int)) -> Result<(), Errno> {
    let handler = unsafe { transmute::<_, sighandler_t>(handler) };
    if unsafe { signal(number, handler) } == SIG_ERR {
        Err(Errno::current())
    } else {
        Ok(())
    }
}
