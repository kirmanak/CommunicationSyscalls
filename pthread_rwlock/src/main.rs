extern crate libc;

mod native;

use native::{create_thread, reverse, inverse, c_void, create_rwlock, unlock_rwlock, read_lock,
             pthread_rwlock_t, count};

use std::time::Duration;
use std::thread::sleep;
use std::env::args;

pub struct ThreadArgument {
    pub characters: *mut Vec<char>,
    pub rwlock: *mut pthread_rwlock_t,
    pub duration: Duration,
}

fn main() {
    let mut args = args().skip(1);
    let interval_main = args.next()
        .expect("Specify the interval of the main thread")
        .parse()
        .expect("Unable to parse the interval of the main thread");
    let interval_sub = args.next()
        .expect("Specify the interval of the writers")
        .parse()
        .expect("Unable to parse the interval of the writers");
    let interval_counter = args.next()
        .expect("Specify the interval of the counter")
        .parse()
        .expect("Unable to parse the interval of the counter");
    let interval_main = Duration::from_micros(interval_main);
    let interval_sub = Duration::from_micros(interval_sub);
    let interval_counter = Duration::from_micros(interval_counter);
    let mut characters = Vec::with_capacity(26);
    for i in 0x61..0x7B {
        characters.push(i as u8 as char);
    }
    let characters_ptr = Box::into_raw(Box::new(characters));
    let rwlock = create_rwlock().expect("Unable to initialize rwlock");
    let rwlock_ptr = Box::into_raw(Box::new(rwlock));
    let writers_arg = ThreadArgument {
        characters: characters_ptr,
        rwlock: rwlock_ptr,
        duration: interval_sub,
    };
    let writers_arg = Box::into_raw(Box::new(writers_arg));
    let counter_arg = ThreadArgument {
        characters: characters_ptr,
        rwlock: rwlock_ptr,
        duration: interval_counter,
    };
    let counter_arg = Box::into_raw(Box::new(counter_arg));
    create_thread(reverse, writers_arg as *mut c_void).expect("Unable to create a thread");
    create_thread(inverse, writers_arg as *mut c_void).expect("Unable to create a thread");
    create_thread(count, counter_arg as *mut c_void).expect("Unable to create a thread");
    let characters = unsafe { Box::from_raw(characters_ptr) };
    loop {
        read_lock(rwlock_ptr).expect("Unable to lock rwlock in main thread");
        println!("{:?}", characters);
        unlock_rwlock(rwlock_ptr).expect("Unable to unlock rwlock in main thread");
        sleep(interval_main);
    }
}
