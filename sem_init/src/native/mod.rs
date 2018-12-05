mod errno;
mod strings;

use self::errno::Errno;
use std::mem::uninitialized;
use std::ptr::null_mut;
use libc::{pthread_t, pthread_create, sem_init, sem_wait, sem_post};
pub use libc::{sem_t, c_void};

use ThreadArgument;

pub extern "C" fn reverse(argument: *mut c_void) -> *mut c_void {
    let argument = unsafe { read_argument(argument) };
    loop {
        wait(argument.sub_semaphore).expect("Unable to wait on sub semaphore in reverse thread");
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
        post(argument.main_semaphore).expect("Unable to post main semaphore in reverse thread");
    }
}

unsafe fn read_argument<'a>(argument: *mut c_void) -> Box<ThreadArgument<'a>> {
    let argument = argument as *mut ThreadArgument;
    let argument = Box::from_raw(argument);
    argument
}

pub fn wait(semaphore: &mut sem_t) -> Result<(), Errno> {
    if unsafe { sem_wait(semaphore) } == 0  {
        Ok(())
    } else {
        Err(Errno::current())
    }
}

pub extern "C" fn inverse(argument: *mut c_void) -> *mut c_void {
    let argument = unsafe { read_argument(argument) };
    loop {
        wait(argument.sub_semaphore).expect("Unable to wait on sub semaphore in inverse thread");
        for i in 0..argument.characters.len() {
            inverse_case(&mut argument.characters[i]);
        }
        post(argument.main_semaphore).expect("Unable to post main semaphore in reverse thread");
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
    let status = unsafe { pthread_create(&mut result, null_mut(), function, argument) };
    if status == 0 {
        Ok(result)
    } else {
        Err(Errno::current())
    }
}

pub fn create_semaphore() -> Result<sem_t, Errno> {
    let mut semaphore: sem_t = unsafe { uninitialized() };
    let status = unsafe { sem_init(&mut semaphore, 0, 0) };
    if status == 0 {
        Ok(semaphore)
    } else {
        Err(Errno::current())
    }
}

pub fn post(semaphore: &mut sem_t) -> Result<(), Errno> {
    if unsafe { sem_post(semaphore) } == 0 {
        Ok(())
    } else {
        Err(Errno::current())
    }
}
