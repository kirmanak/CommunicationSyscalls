extern crate libc;

mod native;

use native::{create_thread, reverse, inverse, c_void, sem_t, create_semaphore, post, wait};

use std::time::Duration;
use std::thread::sleep;

pub struct ThreadArgument<'a> {
    pub characters: &'a mut Vec<char>,
    pub sub_semaphore: &'a mut sem_t,
    pub main_semaphore: &'a mut sem_t,
}

fn main() {
    let duration = Duration::from_secs(1);
    let mut characters = Vec::with_capacity(26);
    for i in 0x61..0x7B {
        characters.push(i as u8 as char);
    }
    let mut sub_semaphore = create_semaphore().expect("Unable to initialize sub semaphore");
    let mut main_semaphore = create_semaphore().expect("Unable to initialize main semaphore");
    let thread_arg = ThreadArgument {
        characters: &mut characters,
        sub_semaphore: &mut sub_semaphore,
        main_semaphore: &mut main_semaphore,
    };
    let thread_arg = Box::new(thread_arg);
    let thread_arg = Box::into_raw(thread_arg);
    create_thread(reverse, thread_arg as *mut c_void).expect("Unable to create a thread");
    create_thread(inverse, thread_arg as *mut c_void).expect("Unable to create a thread");
    let thread_arg = unsafe { Box::from_raw(thread_arg) };
    loop {
        post(thread_arg.sub_semaphore).expect("Unable to post sub semaphore");
        wait(thread_arg.main_semaphore).expect("Unable to wait for main semaphore");
        println!("{:?}", thread_arg.characters);
        sleep(duration);
    }
}
