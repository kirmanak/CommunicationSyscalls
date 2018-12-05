mod errno;
mod strings;

use self::errno::Errno;
use std::mem::uninitialized;
use std::ptr::null_mut;
pub use libc::{c_void, c_int, pthread_mutex_t};
use libc::{pthread_t, pthread_create, pthread_mutex_init, pthread_mutex_lock, pthread_mutex_unlock};
use std::thread::sleep;

use ThreadArgument;

pub extern "C" fn reverse(argument: *mut c_void) -> *mut c_void {
    let argument = unsafe { read_argument(argument) };
    loop {
        lock_mutex(argument.mutex).expect("Unable to lock mutex in reverse thread");
        let mut left = 0;
        let mut right = argument.characters.len() - 1;
        loop {
            if left < right {
                let tmp = argument.characters[left];
                argument.characters[left] = argument.characters[right];
                argument.characters[right] = tmp;
                right -= 1;
                left += 1;
            } else {
                break;
            }
        }
        unlock_mutex(argument.mutex).expect("Unable to unlock mutex in reverse thread");
        sleep(argument.duration);
    }
}

unsafe fn read_argument<'a>(argument: *mut c_void) -> Box<ThreadArgument<'a>> {
    let argument = argument as *mut ThreadArgument;
    let argument = Box::from_raw(argument);
    argument
}

pub extern "C" fn inverse(argument: *mut c_void) -> *mut c_void {
    let argument = unsafe { read_argument(argument) };
    loop {
        lock_mutex(argument.mutex).expect("Unable to lock mutex in inverse thread");
        for i in 0..argument.characters.len() {
            inverse_case(&mut argument.characters[i]);
        }
        unlock_mutex(argument.mutex).expect("Unable to unlock mutex in inverse thread");
        sleep(argument.duration);
    }
}

fn inverse_case(symbol: &mut char) {
    if symbol.is_ascii_uppercase() {
        symbol.make_ascii_lowercase();
    } else {
        symbol.make_ascii_uppercase();
    }
}

pub fn create_thread(
    function: extern "C" fn(_: *mut c_void) -> *mut c_void,
    argument: *mut c_void,
) -> Result<pthread_t, Errno> {
    let mut result: pthread_t = unsafe { uninitialized() };
    if unsafe { pthread_create(&mut result, null_mut(), function, argument) } == 0 {
        Ok(result)
    } else {
        Err(Errno::current())
    }
}

pub fn create_mutex() -> Result<pthread_mutex_t, Errno> {
    let mut mutex = unsafe { uninitialized() };
    if unsafe { pthread_mutex_init(&mut mutex, null_mut()) } == 0 {
        Ok(mutex)
    } else {
        Err(Errno::current())
    }
}

pub fn lock_mutex(mutex: &mut pthread_mutex_t) -> Result<(), Errno> {
    if unsafe { pthread_mutex_lock(mutex) } == 0 {
        Ok(())
    } else {
        Err(Errno::current())
    }
}

pub fn unlock_mutex(mutex: &mut pthread_mutex_t) -> Result<(), Errno> {
    if unsafe { pthread_mutex_unlock(mutex) } == 0 {
        Ok(())
    } else {
        Err(Errno::current())
    }
}
