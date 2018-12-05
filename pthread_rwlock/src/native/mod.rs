mod errno;
mod strings;

use self::errno::Errno;
use std::mem::uninitialized;
use std::ptr::null_mut;
pub use libc::{c_void, c_int, pthread_rwlock_t};
use libc::{pthread_t, pthread_create, pthread_rwlock_init, pthread_rwlock_rdlock,
           pthread_rwlock_wrlock, pthread_rwlock_unlock};
use std::thread::sleep;

use ThreadArgument;

pub extern "C" fn reverse(argument: *mut c_void) -> *mut c_void {
    let argument = unsafe { read_argument(argument) };
    loop {
        write_lock(argument.rwlock).expect("Unable to lock rwlock in reverse thread");
        let mut left = 0;
        let mut characters = unsafe { Box::from_raw(argument.characters) };
        let mut right = characters.len() - 1;
        loop {
            if left < right {
                let tmp = characters[left];
                characters[left] = characters[right];
                characters[right] = tmp;
                right -= 1;
                left += 1;
            } else {
                break;
            }
        }
        unlock_rwlock(argument.rwlock).expect("Unable to unlock rwlock in reverse thread");
        sleep(argument.duration);
    }
}

unsafe fn read_argument<'a>(argument: *mut c_void) -> Box<ThreadArgument> {
    let argument = argument as *mut ThreadArgument;
    let argument = Box::from_raw(argument);
    argument
}

pub extern "C" fn inverse(argument: *mut c_void) -> *mut c_void {
    let argument = unsafe { read_argument(argument) };
    loop {
        write_lock(argument.rwlock).expect("Unable to lock rwlock in inverse thread");
        let mut characters = unsafe { Box::from_raw(argument.characters) };
        for i in 0..characters.len() {
            inverse_case(&mut characters[i]);
        }
        unlock_rwlock(argument.rwlock).expect("Unable to unlock rwlock in inverse thread");
        sleep(argument.duration);
    }
}

pub extern "C" fn count(argument: *mut c_void) -> *mut c_void {
    let argument = unsafe { read_argument(argument) };
    loop {
        read_lock(argument.rwlock).expect("Unable to lock rwlock in counter thread");
        let count = unsafe {
            Box::from_raw(argument.characters)
                .iter()
                .filter(|symbol| symbol.is_ascii_uppercase())
                .count()
        };
        println!("Amount of uppercase symbols: {}", count);
        unlock_rwlock(argument.rwlock).expect("Unable to unlock rwlock in counter thread");
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

pub fn create_rwlock() -> Result<pthread_rwlock_t, Errno> {
    let mut rwlock = unsafe { uninitialized() };
    if unsafe { pthread_rwlock_init(&mut rwlock, null_mut()) } == 0 {
        Ok(rwlock)
    } else {
        Err(Errno::current())
    }
}

pub fn write_lock(rwlock: *mut pthread_rwlock_t) -> Result<(), Errno> {
    if unsafe { pthread_rwlock_wrlock(rwlock) } == 0 {
        Ok(())
    } else {
        Err(Errno::current())
    }
}

pub fn read_lock(rwlock: *mut pthread_rwlock_t) -> Result<(), Errno> {
    if unsafe { pthread_rwlock_rdlock(rwlock) } == 0 {
        Ok(())
    } else {
        Err(Errno::current())
    }
}

pub fn unlock_rwlock(rwlock: *mut pthread_rwlock_t) -> Result<(), Errno> {
    if unsafe { pthread_rwlock_unlock(rwlock) } == 0 {
        Ok(())
    } else {
        Err(Errno::current())
    }
}
