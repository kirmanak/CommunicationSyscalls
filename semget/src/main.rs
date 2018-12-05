extern crate libc;
extern crate simple_signal;

mod native;

use native::{create_thread, reverse, inverse, c_void, c_int, create_semaphore, post,
             remove_semaphore, wait};

use simple_signal::{set_handler, Signal};

use std::time::Duration;
use std::thread::sleep;
use std::process::exit;

pub struct ThreadArgument<'a> {
    pub characters: &'a mut Vec<char>,
    pub set_id: c_int,
}

fn main() {
    let duration = Duration::from_secs(1);
    let mut characters = Vec::with_capacity(26);
    for i in 0x61..0x7B {
        characters.push(i as u8 as char);
    }
    let set_id = create_semaphore().expect("Unable to initialize semaphore");
    let thread_arg = ThreadArgument {
        characters: &mut characters,
        set_id,
    };
    let thread_arg = Box::new(thread_arg);
    let thread_arg = Box::into_raw(thread_arg);
    create_thread(reverse, thread_arg as *mut c_void).expect("Unable to create a thread");
    create_thread(inverse, thread_arg as *mut c_void).expect("Unable to create a thread");
    set_handler(&[Signal::Int, Signal::Term], move |_| {
        remove_semaphore(set_id).expect("Unable to remove semaphores set");
        exit(0);
    });
    let thread_arg = unsafe { Box::from_raw(thread_arg) };
    loop {
        post(set_id, 0).expect("Unable to post semaphore");
        unsafe {
            wait(set_id, 1);
        }
        println!("{:?}", thread_arg.characters);
        sleep(duration);
    }
}
