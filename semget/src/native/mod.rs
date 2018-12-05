mod errno;
mod strings;

use self::errno::Errno;
use std::mem::uninitialized;
use std::ptr::null_mut;
pub use libc::{c_void, c_int};
use libc::{pthread_t, pthread_create};

#[cfg(target_os = "solaris")]
use libc::{key_t, size_t, c_short, c_ushort};
#[cfg(target_os = "solaris")]
const IPC_CREAT: c_int = 0o1000;
#[cfg(target_os = "solaris")]
const IPC_EXCL: c_int = 0o2000;
#[cfg(target_os = "solaris")]
const IPC_PRIVATE: c_int = 0;
#[cfg(target_os = "solaris")]
const IPC_RMID: c_int = 0;
#[cfg(target_os = "solaris")]
#[repr(C)]
struct sembuf {
    sem_num: c_ushort,
    sem_op: c_short,
    sem_flg: c_short,
}
#[cfg(target_os = "solaris")]
extern "C" {
    fn semget(key: key_t, nsems: c_int, semflag: c_int) -> c_int;
    fn semop(semid: c_int, sops: *mut sembuf, nsops: size_t) -> c_int;
    fn semctl(semid: c_int, semnum: c_int, cmd: c_int, ...) -> c_int;
}
#[cfg(not(target_os = "solaris"))]
use libc::{semctl, semget, semop, IPC_CREAT, IPC_EXCL, IPC_PRIVATE, IPC_RMID, sembuf, c_ushort};

use ThreadArgument;

pub extern "C" fn reverse(argument: *mut c_void) -> *mut c_void {
    let argument = unsafe { read_argument(argument) };
    loop {
        unsafe {
            wait(argument.set_id, 0);
        }
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
        post(argument.set_id, 1).expect("Unable to post semaphore 1");
    }
}

unsafe fn read_argument<'a>(argument: *mut c_void) -> Box<ThreadArgument<'a>> {
    let argument = argument as *mut ThreadArgument;
    let argument = Box::from_raw(argument);
    argument
}

pub unsafe fn wait(set_id: c_int, semaphore: c_ushort) {
    let mut operation = sembuf {
        sem_num: semaphore,
        sem_op: -1,
        sem_flg: 0,
    };
    if semop(set_id, &mut operation, 1) == -1 {
        panic!(
            "Something went wrong during sem_wait: {:?}",
            Errno::current()
        );
    }
}

pub extern "C" fn inverse(argument: *mut c_void) -> *mut c_void {
    let argument = unsafe { read_argument(argument) };
    loop {
        unsafe {
            wait(argument.set_id, 0);
        }
        for i in 0..argument.characters.len() {
            inverse_case(&mut argument.characters[i]);
        }
        post(argument.set_id, 1).expect("Unable to post semaphore 1");
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

pub fn create_semaphore() -> Result<c_int, Errno> {
    match unsafe { semget(IPC_PRIVATE, 2, IPC_CREAT | IPC_EXCL | 0o600) } {
        -1 => Err(Errno::current()),
        sem_id => Ok(sem_id),
    }
}

pub fn post(set_id: c_int, semaphore: c_ushort) -> Result<(), Errno> {
    let mut operation = sembuf {
        sem_num: semaphore,
        sem_op: 1,
        sem_flg: 0,
    };
    if unsafe { semop(set_id, &mut operation, 1) } == 0 {
        Ok(())
    } else {
        Err(Errno::current())
    }
}

pub fn remove_semaphore(semaphore: c_int) -> Result<(), Errno> {
    if unsafe { semctl(semaphore, 0, IPC_RMID) } == 0 {
        Ok(())
    } else {
        Err(Errno::current())
    }
}
