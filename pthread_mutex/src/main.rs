extern crate libc;

mod native;

use native::{create_thread, reverse, inverse, c_void, create_mutex, unlock_mutex, lock_mutex,
             pthread_mutex_t};

use std::time::Duration;
use std::thread::sleep;
use std::env::args;

pub struct ThreadArgument<'a> {
    pub characters: &'a mut Vec<char>,
    pub mutex: &'a mut pthread_mutex_t,
    pub duration: Duration,
}

fn main() {
    let mut args = args().skip(1);
    let main_duration = args.next()
        .expect("You've forgotten to specify the main duration")
        .parse()
        .expect("Unable to parse the first duration");
    let sub_duration = args.next()
        .expect("You've forgotten to specify the sub duration")
        .parse()
        .expect("Unable to parse the second duration");
    let main_duration = Duration::from_micros(main_duration);
    let sub_duration = Duration::from_micros(sub_duration);
    let mut characters = Vec::with_capacity(26);
    for i in 0x61..0x7B {
        characters.push(i as u8 as char);
    }
    let mut mutex = create_mutex().expect("Unable to initialize mutex");
    let thread_arg = ThreadArgument {
        characters: &mut characters,
        mutex: &mut mutex,
        duration: sub_duration,
    };
    let thread_arg = Box::new(thread_arg);
    let thread_arg = Box::into_raw(thread_arg);
    create_thread(reverse, thread_arg as *mut c_void).expect("Unable to create a thread");
    create_thread(inverse, thread_arg as *mut c_void).expect("Unable to create a thread");
    let thread_arg = unsafe { Box::from_raw(thread_arg) };
    loop {
        lock_mutex(thread_arg.mutex).expect("Unable to lock mutex in main thread");
        println!("{:?}", thread_arg.characters);
        unlock_mutex(thread_arg.mutex).expect("Unable to unlock mutex in main thread");
        sleep(main_duration);
    }
}
